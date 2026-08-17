//! Oracle parity for the rayon backend (ADR 0001).
//!
//! Everything else verifying the rayon tree compares it against the *CubeCL
//! tree* -- which only shows the migration was faithful, not that the numbers
//! are right. This compares against C libxc 7.0.0 itself, which is the check
//! the project's 1e-12 accuracy constraint is actually about.
//!
//! It also exercises the `POW_1_3` fix: `powf(x, 1.0/3.0)` was replaced with a
//! true cube root, which shifts output for every functional. If that moved the
//! wrong way, this is where it shows.
//!
//! Not gated behind oracle-*: the rayon kernels come in through libxc-reval's
//! own features, not the archived CubeCL families.
//!
//! Run: cargo test --manifest-path verify/Cargo.toml --test rayon_oracle --release

use libxc_core::input::{GgaInput, LdaInput};
use libxc_core::model::{DerivativeOrder, Spin, Thresholds};
use libxc_core::output::{GgaOutput, LdaOutput};
use libxc_core::registry::lookup_by_name;
use libxc_reval::routing;
use libxc_rs_verify::{oracle_gga_all, oracle_lda_all};

/// libxc's own accuracy contract for this project.
const REL_TOL: f64 = 1e-12;

/// Grid points chosen to sit well inside the physical range: the thresholds and
/// the far tails are where libxc and any reimplementation legitimately diverge
/// (different branch cutoffs), and that is not what this test is about.
fn lda_grid() -> Vec<f64> {
    vec![1e-4, 1e-3, 1e-2, 0.1, 0.5, 1.0, 5.0, 20.0]
}

fn gga_grid() -> (Vec<f64>, Vec<f64>) {
    let rho = lda_grid();
    // sigma = |grad rho|^2 with a reduced gradient of order 1.
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

fn worst_rel(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            if x == y {
                0.0
            } else if !x.is_finite() || !y.is_finite() {
                f64::INFINITY
            } else if y.abs() < 1e-280 {
                // Reference is essentially zero; compare absolutely so a tiny
                // denominator does not manufacture a huge relative error.
                (x - y).abs()
            } else {
                ((x - y) / y).abs()
            }
        })
        .fold(0.0f64, f64::max)
}

struct Tally {
    checked: usize,
    passed: usize,
    failed: Vec<(String, &'static str, f64)>,
    unroutable: usize,
    no_id: usize,
}

impl Tally {
    fn new() -> Self {
        Tally { checked: 0, passed: 0, failed: Vec::new(), unroutable: 0, no_id: 0 }
    }
    fn record(&mut self, name: &str, field: &'static str, err: f64) {
        self.checked += 1;
        if err <= REL_TOL {
            self.passed += 1;
        } else {
            self.failed.push((name.to_string(), field, err));
        }
    }
}

#[test]
fn rayon_backend_matches_libxc_oracle() {
    let t = Thresholds::default();
    let mut tally = Tally::new();

    // ---- LDA ------------------------------------------------------------
    let rho = lda_grid();
    let np = rho.len();
    for (fam, name) in routing::SUPPORTED.iter().filter(|(f, _)| *f == "lda") {
        let _ = fam;
        let id = match lookup_by_name(&format!("XC_{name}")) {
            Ok(i) => i.raw() as i32,
            Err(_) => {
                tally.no_id += 1;
                continue;
            }
        };
        let want = match oracle_lda_all(id, 1, &rho) {
            Ok(w) => w,
            Err(_) => continue, // libxc cannot init this one; not our failure
        };
        let (mut zk, mut vrho) = (vec![0f64; np], vec![0f64; np]);
        let (mut v2, mut v3, mut v4) = (vec![0f64; np], vec![0f64; np], vec![0f64; np]);
        // `dispatch_*_by_name` ties `&'a mut output` to `&'a input`, so the
        // mutable borrow of `out` lives as long as `input` does. Both have to
        // go out of scope before the buffers can be read back -- `drop(out)`
        // alone does not end the borrow (this is what the GGA arm below does).
        {
            let input = LdaInput::new(&rho, np, Spin::Unpolarized).expect("lda input");
            let mut out = LdaOutput {
                zk: Some(&mut zk), vrho: Some(&mut vrho),
                v2rho2: Some(&mut v2), v3rho3: Some(&mut v3), v4rho4: Some(&mut v4),
            };
            match routing::dispatch_lda_by_name(name, &input, &mut out, DerivativeOrder::Kxc, Spin::Unpolarized, &t) {
                Some(Ok(())) => {}
                _ => { tally.unroutable += 1; continue; }
            }
        }
        tally.record(name, "zk", worst_rel(&zk, &want.zk));
        tally.record(name, "vrho", worst_rel(&vrho, &want.vrho));
        tally.record(name, "v2rho2", worst_rel(&v2, &want.v2rho2));
    }

    // ---- GGA ------------------------------------------------------------
    let (grho, gsigma) = gga_grid();
    let gnp = grho.len();
    for (_, name) in routing::SUPPORTED.iter().filter(|(f, _)| *f == "gga") {
        let id = match lookup_by_name(&format!("XC_{name}")) {
            Ok(i) => i.raw() as i32,
            Err(_) => { tally.no_id += 1; continue; }
        };
        let want = match oracle_gga_all(id, 1, &grho, &gsigma) {
            Ok(w) => w,
            Err(_) => continue,
        };
        let mut b: Vec<Vec<f64>> = (0..15).map(|_| vec![0f64; gnp]).collect();
        let input = GgaInput::new(&grho, &gsigma, gnp, Spin::Unpolarized).expect("gga input");
        {
            let mut it = b.iter_mut();
            let mut nx = || Some(it.next().unwrap().as_mut_slice());
            let mut out = GgaOutput {
                zk: nx(), vrho: nx(), vsigma: nx(), v2rho2: nx(), v2rhosigma: nx(),
                v2sigma2: nx(), v3rho3: nx(), v3rho2sigma: nx(), v3rhosigma2: nx(),
                v3sigma3: nx(), v4rho4: nx(), v4rho3sigma: nx(), v4rho2sigma2: nx(),
                v4rhosigma3: nx(), v4sigma4: nx(),
            };
            match routing::dispatch_gga_by_name(name, &input, &mut out, DerivativeOrder::Fxc, Spin::Unpolarized, &t) {
                Some(Ok(())) => {}
                _ => { tally.unroutable += 1; continue; }
            }
        }
        tally.record(name, "zk", worst_rel(&b[0], &want.zk));
        tally.record(name, "vrho", worst_rel(&b[1], &want.vrho));
        tally.record(name, "vsigma", worst_rel(&b[2], &want.vsigma));
        tally.record(name, "v2rho2", worst_rel(&b[3], &want.v2rho2));
    }

    // ---- report ---------------------------------------------------------
    println!("\n=== rayon backend vs C libxc 7.0.0 (rel tol {REL_TOL:e}) ===");
    println!("field comparisons : {}", tally.checked);
    println!("  within tol      : {}", tally.passed);
    println!("  over tol        : {}", tally.failed.len());
    println!("not routable      : {}", tally.unroutable);
    println!("no libxc id       : {}", tally.no_id);

    if !tally.failed.is_empty() {
        println!("\nworst offenders:");
        let mut f = tally.failed.clone();
        f.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        for (name, field, err) in f.iter().take(25) {
            println!("  {name:<28} {field:<10} {err:.3e}");
        }
    }

    assert!(
        tally.checked > 0,
        "no comparisons ran -- the harness is not exercising anything"
    );
    assert!(
        tally.failed.is_empty(),
        "{} of {} field comparisons exceeded {REL_TOL:e}",
        tally.failed.len(),
        tally.checked
    );
}
