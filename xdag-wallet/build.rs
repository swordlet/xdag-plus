extern crate winres;
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("cosmic-dark".into())
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("ui/app.slint", config).unwrap();

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./ui/assets/logo.ico");
        res.compile().unwrap();
    }
}
