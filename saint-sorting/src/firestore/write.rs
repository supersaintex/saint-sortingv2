use crate::*;

pub async fn write_firestore<T>(
    session: Session,
    document_id: String,
    obj: &T,
    tmpl: web::Data<Tera>,) 
    -> actix_web::Result<HttpResponse, super::firestore_error::FireStoreError>
where
    T: Serialize,
    {
    //session check & unwrap user_id
    let user_id = match session.get::<Uuid>("user_id")? {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(i) => i.to_string()
    };

    let cred = Credentials::from_file("firebase-service-account.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let _result = documents::write(&auth, &user_id, Some(document_id), &obj, documents::WriteOptions::default());

    let context = Context::new();
    let view = tmpl.render("db_top.html", &context)
       .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("texthtml").body(view))
}

#[derive(Serialize, Deserialize)]
pub struct FormParamsDbWrite {
    document_id: String,
    a_string: String,
    an_int: u32,
    another_int: u32,
}
