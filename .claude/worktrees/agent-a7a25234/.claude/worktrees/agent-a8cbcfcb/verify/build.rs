use std::env;
use std::path::PathBuf;

fn main() {
    // Build vendored libxc 7.0.0 via cmake
    let dst = cmake::Config::new("../libxc-master")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("ENABLE_FORTRAN", "OFF")
        .define("ENABLE_PYTHON", "OFF")
        .define("DISABLE_VXC", "OFF")
        .define("DISABLE_FXC", "OFF")
        .define("DISABLE_KXC", "OFF")
        .define("DISABLE_LXC", "OFF")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .build();

    // Link against the built static library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=xc");

    // Also check lib64 (some cmake builds put it there)
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());

    // Generate Rust FFI bindings from xc.h
    let header_path = dst.join("include").join("xc.h");
    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .allowlist_function("xc_.*")
        .allowlist_type("xc_.*")
        .allowlist_var("XC_.*")
        .derive_default(true)
        .generate()
        .expect("Failed to generate bindings for libxc");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libxc_bindings.rs"))
        .expect("Failed to write bindings");
}
