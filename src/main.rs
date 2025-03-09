fn main() -> rquickjs::Result<()> {
    let runtime = rquickjs::Runtime::new()?;
    let context = rquickjs::Context::full(&runtime)?;
    let string = context.with(|ctx| ctx.eval::<String, _>("'Hello, World!'"))?;
    println!("{string}");
    Ok(())
}
