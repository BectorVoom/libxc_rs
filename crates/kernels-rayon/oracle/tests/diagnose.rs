//! Pointwise dump of this tree against C libxc, for one functional at a time.
//!
//! `rayon_oracle` answers "which functionals disagree, and by how much".
//! Answering *why* needs the actual numbers side by side, because a relative
//! error tells you nothing about whether the value is scaled, shifted, sign-
//! flipped, or right-except-at-one-point.
//!
//! Not a pass/fail test -- it asserts nothing. Run it and read it:
//!
//! ```text
//! cargo test --release --manifest-path crates/kernels-rayon/oracle/Cargo.toml \
//!     --test diagnose -- --nocapture
//! ```
//!
//! Set `XCDIAG` to a comma-separated list to override which functionals it
//! dumps.

use libxc_core::input::GgaInput;
use libxc_core::model::{DerivativeOrder, Spin, Thresholds};
use libxc_core::output::GgaOutput;
use libxc_core::registry::lookup_by_name;
use libxc_reval::routing;
use libxc_rs_verify::oracle_gga_all;

/// The same grid `rayon_oracle` uses, so a discrepancy seen there reappears here.
fn gga_grid() -> (Vec<f64>, Vec<f64>) {
    let rho = vec![1e-4, 1e-3, 1e-2, 0.1, 0.5, 1.0, 5.0, 20.0];
    let sigma: Vec<f64> = rho
        .iter()
        .map(|r| {
            let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI * r).powf(1.0 / 3.0);
            let g = 2.0 * kf * r;
            g * g
        })
        .collect();
    (rho, sigma)
}

#[test]
fn dump_gga_disagreements() {
    let names: Vec<String> = std::env::var("XCDIAG")
        .unwrap_or_else(|_| {
            "gga_x_fd_lb94,gga_x_fd_revlb94,gga_k_absp4,gga_k_tfvw,gga_k_absp1,gga_x_kgg99"
                .to_string()
        })
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let (rho, sigma) = gga_grid();
    let np = rho.len();
    let t = Thresholds::default();

    for name in &names {
        println!("\n================ {name} ================");
        let id = match lookup_by_name(&format!("XC_{name}")) {
            Ok(i) => i.raw() as i32,
            Err(e) => {
                println!("  no libxc id: {e}");
                continue;
            }
        };
        let want = match oracle_gga_all(id, 1, &rho, &sigma) {
            Ok(w) => w,
            Err(e) => {
                println!("  libxc refused to init: {e}");
                continue;
            }
        };
        let mut b: Vec<Vec<f64>> = (0..15).map(|_| vec![0f64; np]).collect();
        {
            let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).expect("input");
            let mut it = b.iter_mut();
            let mut nx = || Some(it.next().unwrap().as_mut_slice());
            let mut out = GgaOutput {
                zk: nx(), vrho: nx(), vsigma: nx(), v2rho2: nx(), v2rhosigma: nx(),
                v2sigma2: nx(), v3rho3: nx(), v3rho2sigma: nx(), v3rhosigma2: nx(),
                v3sigma3: nx(), v4rho4: nx(), v4rho3sigma: nx(), v4rho2sigma2: nx(),
                v4rhosigma3: nx(), v4sigma4: nx(),
            };
            match routing::dispatch_gga_by_name(
                name, &input, &mut out, DerivativeOrder::Fxc, Spin::Unpolarized, &t)
            {
                Some(Ok(())) => {}
                Some(Err(e)) => { println!("  dispatch error: {e:?}"); continue; }
                None => { println!("  not routed (UNSUPPORTED)"); continue; }
            }
        }

        for (field, got, exp) in [
            ("zk", &b[0], &want.zk),
            ("vrho", &b[1], &want.vrho),
            ("vsigma", &b[2], &want.vsigma),
        ] {
            println!("  {field}:");
            println!("      {:>10} {:>23} {:>23} {:>11} {:>9}",
                     "rho", "ours", "libxc", "rel", "ours/libxc");
            for i in 0..np {
                let (g, e) = (got[i], exp[i]);
                let rel = if e == 0.0 { f64::NAN } else { ((g - e) / e).abs() };
                let ratio = if e == 0.0 { f64::NAN } else { g / e };
                let flag = if rel > 1e-12 { " <<<" } else { "" };
                println!("      {:>10.0e} {:>23.15e} {:>23.15e} {:>11.2e} {:>9.4}{flag}",
                         rho[i], g, e, rel, ratio);
            }
        }
    }
}
