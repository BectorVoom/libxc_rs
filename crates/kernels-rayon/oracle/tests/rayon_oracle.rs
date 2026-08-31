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

/// The project's accuracy contract: **energy** relative error <= 1e-12 against
/// the libxc oracle (`CLAUDE.md`). Applied to `zk` and to the first
/// derivative w.r.t. rho, both of which meet it on every routed functional.
const REL_TOL: f64 = 1e-12;

/// The floor for `vsigma` and `v2rho2`.
///
/// These are not held to `REL_TOL`, and the reason is a property of the
/// comparison rather than of this tree. The oracle is libxc *as GCC compiles
/// it*; we are rustc. Both evaluate the same maple2c expression sequence, but
/// GCC contracts `a*b+c` into FMA by default and rustc does not, so identical
/// formulas produce results differing in the last bits -- and `vsigma` and
/// `v2rho2` reach the end of long chains where that divergence has been
/// amplified by cancellation.
///
/// Everything that *could* be a defect here was checked against libxc's source
/// and ruled out, functional by functional, for the nine cases that sat above
/// `REL_TOL`: the numeric literals and call counts in the fxc bodies match the
/// maple2c source exactly; the `ext_params` values match (including ones libxc
/// computes, like `1.43169/X_FACTOR_C`, which agrees bit-for-bit); the
/// `POW_n_3` helpers match libxc's macros including their left-to-right
/// grouping, which is not associative in floating point; and every math
/// function the kernels call is 0 ulp against glibc. Rebuilding the oracle's
/// libxc with `-ffp-contract=off` moves every one of the nine and drops two
/// below `REL_TOL` outright, which is the direct evidence for the mechanism.
///
/// `1e-10` sits above the worst observed (`hyb_gga_xc_wb97x_d3` `v2rho2` at
/// 4.7e-11) with room, and far below anything a real defect has produced here
/// -- the four fixed in this harness's history showed up at 1e-7 and worse.
/// The per-field worst error is printed on every run regardless, so drift
/// inside the band stays visible.
const DERIV_TOL: f64 = 1e-10;

/// Tolerance for one output field.
fn tol_for(field: &str) -> f64 {
    match field {
        "vsigma" | "v2rho2" => DERIV_TOL,
        _ => REL_TOL,
    }
}

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

/// Worst elementwise relative disagreement, skipping elements where *both*
/// sides are numerically zero for this functional.
///
/// `scale` is the largest `|zk|` libxc produced for this functional on this
/// grid -- the size of the energy density every other output is a derivative
/// of. An element whose value is below `scale * ZERO_FRAC` on *both* sides
/// carries no significant digits, and the two implementations are simply
/// reporting their own rounding dust for a quantity that is zero.
///
/// `gga_k_tfvw` and `gga_k_absp4` both have an identically-zero `vrho`.
/// libxc's answer for `gga_k_tfvw` is
/// `2.6e-20, 2.1e-17, -4.7e-17, -4.0e-16, -8.9e-16, 0.0, 6.2e-15, 3.0e-14` --
/// exact `0.0` at one grid point and noise at the others, against a `|zk|` of
/// order 1. Dividing our noise by libxc's noise scored that 1.55, which says
/// nothing about either implementation. The previous guard only caught an
/// exactly-zero reference (`|y| < 1e-280`), which this is not.
///
/// This deliberately does *not* relax the comparison for any element that
/// carries magnitude: an output that is small but meaningful (`vsigma` is
/// 3.3e-6 where `zk` is 2.7e-2, twelve orders above the cutoff) is still held
/// to the full relative tolerance. Only "both sides say zero" is exempt.
fn worst_rel(a: &[f64], b: &[f64], scale: f64) -> f64 {
    /// Fraction of a functional's own `|zk|` below which a value cannot carry
    /// significant digits: double precision reaches ~1e-16 relative, so a
    /// value 1e-12 of the scale is already at the noise floor of the
    /// intermediate arithmetic that produced it.
    const ZERO_FRAC: f64 = 1e-12;
    let zero = scale * ZERO_FRAC;
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            if x == y {
                0.0
            } else if !x.is_finite() || !y.is_finite() {
                f64::INFINITY
            } else if x.abs().max(y.abs()) <= zero {
                // Both are zero to the precision this functional can express.
                0.0
            } else if y.abs() < 1e-280 {
                (x - y).abs()
            } else {
                ((x - y) / y).abs()
            }
        })
        .fold(0.0f64, f64::max)
}

/// The scale every field of one functional is judged against: the largest
/// `|zk|` on the grid, or `1.0` if the reference energy density is itself zero.
fn zk_scale(zk: &[f64]) -> f64 {
    let m = zk.iter().fold(0.0f64, |acc, v| acc.max(v.abs()));
    if m > 0.0 { m } else { 1.0 }
}

struct Tally {
    checked: usize,
    passed: usize,
    failed: Vec<(String, &'static str, f64)>,
    /// Worst error seen per field and the functional that produced it, kept
    /// even when it passes: a field creeping up inside its band is the early
    /// warning this harness would otherwise not give.
    worst: std::collections::BTreeMap<&'static str, (f64, String)>,
    unroutable: usize,
    no_id: usize,
}

impl Tally {
    fn new() -> Self {
        Tally { checked: 0, passed: 0, failed: Vec::new(),
                worst: std::collections::BTreeMap::new(), unroutable: 0, no_id: 0 }
    }
    fn record(&mut self, name: &str, field: &'static str, err: f64) {
        self.checked += 1;
        let w = self.worst.entry(field).or_insert((0.0, String::new()));
        if err > w.0 {
            *w = (err, name.to_string());
        }
        if err <= tol_for(field) {
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
        let sc = zk_scale(&want.zk);
        tally.record(name, "zk", worst_rel(&zk, &want.zk, sc));
        tally.record(name, "vrho", worst_rel(&vrho, &want.vrho, sc));
        tally.record(name, "v2rho2", worst_rel(&v2, &want.v2rho2, sc));
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
        let sc = zk_scale(&want.zk);
        tally.record(name, "zk", worst_rel(&b[0], &want.zk, sc));
        tally.record(name, "vrho", worst_rel(&b[1], &want.vrho, sc));
        tally.record(name, "vsigma", worst_rel(&b[2], &want.vsigma, sc));
        tally.record(name, "v2rho2", worst_rel(&b[3], &want.v2rho2, sc));
    }

    // ---- report ---------------------------------------------------------
    println!("\n=== rayon backend vs C libxc 7.0.0 ===");
    println!("tolerance: zk/vrho {REL_TOL:e} (energy contract), \
              vsigma/v2rho2 {DERIV_TOL:e} (compiler-codegen floor)");
    println!("field comparisons : {}", tally.checked);
    println!("  within tol      : {}", tally.passed);
    println!("  over tol        : {}", tally.failed.len());
    println!("not routable      : {}", tally.unroutable);
    println!("no libxc id       : {}", tally.no_id);

    println!("\nworst per field (shown whether or not it passes):");
    for (field, (err, who)) in &tally.worst {
        let t = tol_for(field);
        println!("  {field:<10} {err:.3e}  ({who})   tol {t:.0e}  {}",
                 if *err <= t { "ok" } else { "OVER" });
    }

    if !tally.failed.is_empty() {
        // Every failure, not a truncated head: the tail is where the
        // marginal 1e-11-ish cases live, and telling those apart from the
        // percent-level structural ones is the whole diagnostic value.
        println!("\nall offenders (worst first):");
        let mut f = tally.failed.clone();
        f.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        for (name, field, err) in f.iter() {
            println!("  {name:<28} {field:<10} {err:.3e}");
        }
    }

    assert!(
        tally.checked > 0,
        "no comparisons ran -- the harness is not exercising anything"
    );
    assert!(
        tally.failed.is_empty(),
        "{} of {} field comparisons exceeded their tolerance \
         (zk/vrho {REL_TOL:e}, vsigma/v2rho2 {DERIV_TOL:e})",
        tally.failed.len(),
        tally.checked
    );
}
