//! Batch oracle verification for all LDA functionals.
//!
//! Verifies that the C libxc oracle can successfully evaluate all LDA functionals.
//! Per-functional Rust-vs-oracle comparison tests will be activated as kernels are translated.

use libxc_rs_verify::{oracle_lda_all, oracle_func_flags, FLAGS_HAVE_EXC};

struct FunctionalTestCase {
    id: i32,
    name: &'static str,
}

/// All 67 LDA functionals from libxc 7.0.0 (xc_funcs.h).
const LDA_FUNCTIONALS: &[FunctionalTestCase] = &[
    FunctionalTestCase { id: 1, name: "lda_x" },
    FunctionalTestCase { id: 2, name: "lda_c_wigner" },
    FunctionalTestCase { id: 3, name: "lda_c_rpa" },
    FunctionalTestCase { id: 4, name: "lda_c_hl" },
    FunctionalTestCase { id: 5, name: "lda_c_gl" },
    FunctionalTestCase { id: 6, name: "lda_c_xalpha" },
    FunctionalTestCase { id: 7, name: "lda_c_vwn" },
    FunctionalTestCase { id: 8, name: "lda_c_vwn_rpa" },
    FunctionalTestCase { id: 9, name: "lda_c_pz" },
    FunctionalTestCase { id: 10, name: "lda_c_pz_mod" },
    FunctionalTestCase { id: 11, name: "lda_c_ob_pz" },
    FunctionalTestCase { id: 12, name: "lda_c_pw" },
    FunctionalTestCase { id: 13, name: "lda_c_pw_mod" },
    FunctionalTestCase { id: 14, name: "lda_c_ob_pw" },
    FunctionalTestCase { id: 15, name: "lda_c_2d_amgb" },
    FunctionalTestCase { id: 16, name: "lda_c_2d_prm" },
    FunctionalTestCase { id: 17, name: "lda_c_vbh" },
    FunctionalTestCase { id: 18, name: "lda_c_1d_csc" },
    FunctionalTestCase { id: 19, name: "lda_x_2d" },
    FunctionalTestCase { id: 20, name: "lda_xc_teter93" },
    FunctionalTestCase { id: 21, name: "lda_x_1d_soft" },
    FunctionalTestCase { id: 22, name: "lda_c_ml1" },
    FunctionalTestCase { id: 23, name: "lda_c_ml2" },
    FunctionalTestCase { id: 24, name: "lda_c_gombas" },
    FunctionalTestCase { id: 25, name: "lda_c_pw_rpa" },
    FunctionalTestCase { id: 26, name: "lda_c_1d_loos" },
    FunctionalTestCase { id: 27, name: "lda_c_rc04" },
    FunctionalTestCase { id: 28, name: "lda_c_vwn_1" },
    FunctionalTestCase { id: 29, name: "lda_c_vwn_2" },
    FunctionalTestCase { id: 30, name: "lda_c_vwn_3" },
    FunctionalTestCase { id: 31, name: "lda_c_vwn_4" },
    FunctionalTestCase { id: 43, name: "lda_xc_zlp" },
    FunctionalTestCase { id: 50, name: "lda_k_tf" },
    FunctionalTestCase { id: 51, name: "lda_k_lp" },
    FunctionalTestCase { id: 259, name: "lda_xc_ksdt" },
    FunctionalTestCase { id: 287, name: "lda_c_chachiyo" },
    FunctionalTestCase { id: 289, name: "lda_c_lp96" },
    FunctionalTestCase { id: 307, name: "lda_c_chachiyo_mod" },
    FunctionalTestCase { id: 308, name: "lda_c_karasiev_mod" },
    FunctionalTestCase { id: 317, name: "lda_c_w20" },
    FunctionalTestCase { id: 318, name: "lda_xc_corrksdt" },
    FunctionalTestCase { id: 532, name: "lda_x_rel" },
    FunctionalTestCase { id: 536, name: "lda_xc_1d_ehwlrg_1" },
    FunctionalTestCase { id: 537, name: "lda_xc_1d_ehwlrg_2" },
    FunctionalTestCase { id: 538, name: "lda_xc_1d_ehwlrg_3" },
    FunctionalTestCase { id: 546, name: "lda_x_erf" },
    FunctionalTestCase { id: 547, name: "lda_xc_lp_a" },
    FunctionalTestCase { id: 548, name: "lda_xc_lp_b" },
    FunctionalTestCase { id: 549, name: "lda_x_rae" },
    FunctionalTestCase { id: 550, name: "lda_k_zlp" },
    FunctionalTestCase { id: 551, name: "lda_c_mcweeny" },
    FunctionalTestCase { id: 552, name: "lda_c_br78" },
    FunctionalTestCase { id: 554, name: "lda_c_pk09" },
    FunctionalTestCase { id: 573, name: "lda_c_ow_lyp" },
    FunctionalTestCase { id: 574, name: "lda_c_ow" },
    FunctionalTestCase { id: 577, name: "lda_xc_gdsmfb" },
    FunctionalTestCase { id: 578, name: "lda_c_gk72" },
    FunctionalTestCase { id: 579, name: "lda_c_karasiev" },
    FunctionalTestCase { id: 580, name: "lda_k_lp96" },
    FunctionalTestCase { id: 590, name: "lda_c_pmgb06" },
    FunctionalTestCase { id: 599, name: "lda_xc_tih" },
    FunctionalTestCase { id: 600, name: "lda_x_1d_exponential" },
    FunctionalTestCase { id: 641, name: "lda_x_yukawa" },
    FunctionalTestCase { id: 654, name: "lda_c_pw_erf" },
    FunctionalTestCase { id: 683, name: "lda_c_upw92" },
    FunctionalTestCase { id: 684, name: "lda_c_rpw92" },
    FunctionalTestCase { id: 692, name: "lda_x_sloc" },
];

// Tolerance tiers per D-10
#[allow(dead_code)]
const TOL_EXC: f64 = 1e-12;
#[allow(dead_code)]
const TOL_VXC: f64 = 1e-10;
#[allow(dead_code)]
const TOL_FXC: f64 = 1e-8;
#[allow(dead_code)]
const TOL_KXC: f64 = 1e-6;
#[allow(dead_code)]
const TOL_LXC: f64 = 1e-4;

// Test data: representative densities for both unpolarized and polarized modes
const RHO_UNPOL: &[f64] = &[0.1, 0.5, 1.0, 5.0];
const RHO_POL: &[f64] = &[0.1, 0.05, 0.5, 0.3, 1.0, 0.8, 5.0, 3.0];

/// Verify oracle calls succeed for all LDA functionals (unpolarized).
#[test]
fn test_all_lda_oracle_unpol() {
    let mut failures = Vec::new();
    let mut skipped = 0;
    for tc in LDA_FUNCTIONALS {
        // Skip functionals that don't support Exc (would cause libxc to exit)
        let flags = oracle_func_flags(tc.id, 1).unwrap_or(0);
        if flags & FLAGS_HAVE_EXC == 0 {
            eprintln!("SKIP {}: no EXC support", tc.name);
            skipped += 1;
            continue;
        }
        match oracle_lda_all(tc.id, 1, RHO_UNPOL) {
            Ok(oracle) => {
                assert!(!oracle.zk.is_empty(), "{}: zk empty", tc.name);
            }
            Err(e) => {
                failures.push(format!("{} (id={}): {e}", tc.name, tc.id));
            }
        }
    }
    if !failures.is_empty() {
        panic!(
            "LDA oracle unpolarized failures ({}/{}):\n  {}",
            failures.len(),
            LDA_FUNCTIONALS.len(),
            failures.join("\n  ")
        );
    }
    eprintln!(
        "LDA oracle unpolarized: {}/{} functionals OK ({} skipped, no EXC)",
        LDA_FUNCTIONALS.len() - skipped,
        LDA_FUNCTIONALS.len(),
        skipped,
    );
}

/// Verify oracle calls succeed for all LDA functionals (polarized).
#[test]
fn test_all_lda_oracle_pol() {
    let mut failures = Vec::new();
    let mut skipped = 0;
    for tc in LDA_FUNCTIONALS {
        let flags = oracle_func_flags(tc.id, 2).unwrap_or(0);
        if flags & FLAGS_HAVE_EXC == 0 {
            eprintln!("SKIP {}: no EXC support", tc.name);
            skipped += 1;
            continue;
        }
        match oracle_lda_all(tc.id, 2, RHO_POL) {
            Ok(oracle) => {
                assert!(!oracle.zk.is_empty(), "{}: zk empty", tc.name);
            }
            Err(e) => {
                failures.push(format!("{} (id={}): {e}", tc.name, tc.id));
            }
        }
    }
    if !failures.is_empty() {
        panic!(
            "LDA oracle polarized failures ({}/{}):\n  {}",
            failures.len(),
            LDA_FUNCTIONALS.len(),
            failures.join("\n  ")
        );
    }
    eprintln!(
        "LDA oracle polarized: {}/{} functionals OK ({} skipped, no EXC)",
        LDA_FUNCTIONALS.len() - skipped,
        LDA_FUNCTIONALS.len(),
        skipped,
    );
}
