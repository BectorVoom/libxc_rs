//! Batch oracle verification for all GGA functionals.
//!
//! Verifies that the C libxc oracle can successfully evaluate all GGA functionals.
//! Per-functional Rust-vs-oracle comparison tests will be activated as kernels are translated.

use libxc_rs_verify::{oracle_gga_all, oracle_func_flags, FLAGS_HAVE_EXC};

struct FunctionalTestCase {
    id: i32,
    name: &'static str,
}

/// All 256 GGA functionals from libxc 7.0.0 (xc_funcs.h).
const GGA_FUNCTIONALS: &[FunctionalTestCase] = &[
    FunctionalTestCase { id: 32, name: "gga_x_gam" },
    FunctionalTestCase { id: 33, name: "gga_c_gam" },
    FunctionalTestCase { id: 34, name: "gga_x_hcth_a" },
    FunctionalTestCase { id: 35, name: "gga_x_ev93" },
    FunctionalTestCase { id: 38, name: "gga_x_bcgp" },
    FunctionalTestCase { id: 39, name: "gga_c_acgga" },
    FunctionalTestCase { id: 40, name: "gga_x_lambda_oc2_n" },
    FunctionalTestCase { id: 41, name: "gga_x_b86_r" },
    FunctionalTestCase { id: 44, name: "gga_x_lambda_ch_n" },
    FunctionalTestCase { id: 45, name: "gga_x_lambda_lo_n" },
    FunctionalTestCase { id: 46, name: "gga_x_hjs_b88_v2" },
    FunctionalTestCase { id: 47, name: "gga_c_q2d" },
    FunctionalTestCase { id: 48, name: "gga_x_q2d" },
    FunctionalTestCase { id: 49, name: "gga_x_pbe_mol" },
    FunctionalTestCase { id: 52, name: "gga_k_tfvw" },
    FunctionalTestCase { id: 53, name: "gga_k_revapbeint" },
    FunctionalTestCase { id: 54, name: "gga_k_apbeint" },
    FunctionalTestCase { id: 55, name: "gga_k_revapbe" },
    FunctionalTestCase { id: 56, name: "gga_x_ak13" },
    FunctionalTestCase { id: 57, name: "gga_k_meyer" },
    FunctionalTestCase { id: 58, name: "gga_x_lv_rpw86" },
    FunctionalTestCase { id: 59, name: "gga_x_pbe_tca" },
    FunctionalTestCase { id: 60, name: "gga_x_pbeint" },
    FunctionalTestCase { id: 61, name: "gga_c_zpbeint" },
    FunctionalTestCase { id: 62, name: "gga_c_pbeint" },
    FunctionalTestCase { id: 63, name: "gga_c_zpbesol" },
    FunctionalTestCase { id: 65, name: "gga_xc_opbe_d" },
    FunctionalTestCase { id: 66, name: "gga_xc_opwlyp_d" },
    FunctionalTestCase { id: 67, name: "gga_xc_oblyp_d" },
    FunctionalTestCase { id: 68, name: "gga_x_vmt84_ge" },
    FunctionalTestCase { id: 69, name: "gga_x_vmt84_pbe" },
    FunctionalTestCase { id: 70, name: "gga_x_vmt_ge" },
    FunctionalTestCase { id: 71, name: "gga_x_vmt_pbe" },
    FunctionalTestCase { id: 79, name: "gga_c_n12_sx" },
    FunctionalTestCase { id: 80, name: "gga_c_n12" },
    FunctionalTestCase { id: 82, name: "gga_x_n12" },
    FunctionalTestCase { id: 83, name: "gga_c_regtpss" },
    FunctionalTestCase { id: 84, name: "gga_c_op_xalpha" },
    FunctionalTestCase { id: 85, name: "gga_c_op_g96" },
    FunctionalTestCase { id: 86, name: "gga_c_op_pbe" },
    FunctionalTestCase { id: 87, name: "gga_c_op_b88" },
    FunctionalTestCase { id: 88, name: "gga_c_ft97" },
    FunctionalTestCase { id: 89, name: "gga_c_spbe" },
    FunctionalTestCase { id: 90, name: "gga_x_ssb_sw" },
    FunctionalTestCase { id: 91, name: "gga_x_ssb" },
    FunctionalTestCase { id: 92, name: "gga_x_ssb_d" },
    FunctionalTestCase { id: 93, name: "gga_xc_hcth_407p" },
    FunctionalTestCase { id: 94, name: "gga_xc_hcth_p76" },
    FunctionalTestCase { id: 95, name: "gga_xc_hcth_p14" },
    FunctionalTestCase { id: 96, name: "gga_xc_b97_gga1" },
    FunctionalTestCase { id: 97, name: "gga_c_hcth_a" },
    FunctionalTestCase { id: 98, name: "gga_x_bpccac" },
    FunctionalTestCase { id: 99, name: "gga_c_revtca" },
    FunctionalTestCase { id: 100, name: "gga_c_tca" },
    FunctionalTestCase { id: 101, name: "gga_x_pbe" },
    FunctionalTestCase { id: 102, name: "gga_x_pbe_r" },
    FunctionalTestCase { id: 103, name: "gga_x_b86" },
    FunctionalTestCase { id: 105, name: "gga_x_b86_mgc" },
    FunctionalTestCase { id: 106, name: "gga_x_b88" },
    FunctionalTestCase { id: 107, name: "gga_x_g96" },
    FunctionalTestCase { id: 108, name: "gga_x_pw86" },
    FunctionalTestCase { id: 109, name: "gga_x_pw91" },
    FunctionalTestCase { id: 110, name: "gga_x_optx" },
    FunctionalTestCase { id: 111, name: "gga_x_dk87_r1" },
    FunctionalTestCase { id: 112, name: "gga_x_dk87_r2" },
    FunctionalTestCase { id: 113, name: "gga_x_lg93" },
    FunctionalTestCase { id: 114, name: "gga_x_ft97_a" },
    FunctionalTestCase { id: 115, name: "gga_x_ft97_b" },
    FunctionalTestCase { id: 116, name: "gga_x_pbe_sol" },
    FunctionalTestCase { id: 117, name: "gga_x_rpbe" },
    FunctionalTestCase { id: 118, name: "gga_x_wc" },
    FunctionalTestCase { id: 119, name: "gga_x_mpw91" },
    FunctionalTestCase { id: 120, name: "gga_x_am05" },
    FunctionalTestCase { id: 121, name: "gga_x_pbea" },
    FunctionalTestCase { id: 122, name: "gga_x_mpbe" },
    FunctionalTestCase { id: 123, name: "gga_x_xpbe" },
    FunctionalTestCase { id: 124, name: "gga_x_2d_b86_mgc" },
    FunctionalTestCase { id: 125, name: "gga_x_bayesian" },
    FunctionalTestCase { id: 126, name: "gga_x_pbe_jsjr" },
    FunctionalTestCase { id: 127, name: "gga_x_2d_b88" },
    FunctionalTestCase { id: 128, name: "gga_x_2d_b86" },
    FunctionalTestCase { id: 129, name: "gga_x_2d_pbe" },
    FunctionalTestCase { id: 130, name: "gga_c_pbe" },
    FunctionalTestCase { id: 131, name: "gga_c_lyp" },
    FunctionalTestCase { id: 132, name: "gga_c_p86" },
    FunctionalTestCase { id: 133, name: "gga_c_pbe_sol" },
    FunctionalTestCase { id: 134, name: "gga_c_pw91" },
    FunctionalTestCase { id: 135, name: "gga_c_am05" },
    FunctionalTestCase { id: 136, name: "gga_c_xpbe" },
    FunctionalTestCase { id: 137, name: "gga_c_lm" },
    FunctionalTestCase { id: 138, name: "gga_c_pbe_jrgx" },
    FunctionalTestCase { id: 139, name: "gga_x_optb88_vdw" },
    FunctionalTestCase { id: 140, name: "gga_x_pbek1_vdw" },
    FunctionalTestCase { id: 141, name: "gga_x_optpbe_vdw" },
    FunctionalTestCase { id: 142, name: "gga_x_rge2" },
    FunctionalTestCase { id: 143, name: "gga_c_rge2" },
    FunctionalTestCase { id: 144, name: "gga_x_rpw86" },
    FunctionalTestCase { id: 145, name: "gga_x_kt1" },
    FunctionalTestCase { id: 146, name: "gga_xc_kt2" },
    FunctionalTestCase { id: 147, name: "gga_c_wl" },
    FunctionalTestCase { id: 148, name: "gga_c_wi" },
    FunctionalTestCase { id: 149, name: "gga_x_mb88" },
    FunctionalTestCase { id: 150, name: "gga_x_sogga" },
    FunctionalTestCase { id: 151, name: "gga_x_sogga11" },
    FunctionalTestCase { id: 152, name: "gga_c_sogga11" },
    FunctionalTestCase { id: 153, name: "gga_c_wi0" },
    FunctionalTestCase { id: 154, name: "gga_xc_th1" },
    FunctionalTestCase { id: 155, name: "gga_xc_th2" },
    FunctionalTestCase { id: 156, name: "gga_xc_th3" },
    FunctionalTestCase { id: 157, name: "gga_xc_th4" },
    FunctionalTestCase { id: 158, name: "gga_x_c09x" },
    FunctionalTestCase { id: 159, name: "gga_c_sogga11_x" },
    FunctionalTestCase { id: 160, name: "gga_x_lb" },
    FunctionalTestCase { id: 161, name: "gga_xc_hcth_93" },
    FunctionalTestCase { id: 162, name: "gga_xc_hcth_120" },
    FunctionalTestCase { id: 163, name: "gga_xc_hcth_147" },
    FunctionalTestCase { id: 164, name: "gga_xc_hcth_407" },
    FunctionalTestCase { id: 165, name: "gga_xc_edf1" },
    FunctionalTestCase { id: 166, name: "gga_xc_xlyp" },
    FunctionalTestCase { id: 167, name: "gga_xc_kt1" },
    FunctionalTestCase { id: 168, name: "gga_x_lspbe" },
    FunctionalTestCase { id: 169, name: "gga_x_lsrpbe" },
    FunctionalTestCase { id: 170, name: "gga_xc_b97_d" },
    FunctionalTestCase { id: 171, name: "gga_x_optb86b_vdw" },
    FunctionalTestCase { id: 173, name: "gga_xc_pbe1w" },
    FunctionalTestCase { id: 174, name: "gga_xc_mpwlyp1w" },
    FunctionalTestCase { id: 175, name: "gga_xc_pbelyp1w" },
    FunctionalTestCase { id: 176, name: "gga_c_acggap" },
    FunctionalTestCase { id: 179, name: "gga_x_b88_6311g" },
    FunctionalTestCase { id: 180, name: "gga_x_ncap" },
    FunctionalTestCase { id: 181, name: "gga_xc_ncap" },
    FunctionalTestCase { id: 182, name: "gga_x_lbm" },
    FunctionalTestCase { id: 183, name: "gga_x_ol2" },
    FunctionalTestCase { id: 184, name: "gga_x_apbe" },
    FunctionalTestCase { id: 185, name: "gga_k_apbe" },
    FunctionalTestCase { id: 186, name: "gga_c_apbe" },
    FunctionalTestCase { id: 187, name: "gga_k_tw1" },
    FunctionalTestCase { id: 188, name: "gga_k_tw2" },
    FunctionalTestCase { id: 189, name: "gga_k_tw3" },
    FunctionalTestCase { id: 190, name: "gga_k_tw4" },
    FunctionalTestCase { id: 191, name: "gga_x_htbs" },
    FunctionalTestCase { id: 192, name: "gga_x_airy" },
    FunctionalTestCase { id: 193, name: "gga_x_lag" },
    FunctionalTestCase { id: 194, name: "gga_xc_mohlyp" },
    FunctionalTestCase { id: 195, name: "gga_xc_mohlyp2" },
    FunctionalTestCase { id: 196, name: "gga_xc_th_fl" },
    FunctionalTestCase { id: 197, name: "gga_xc_th_fc" },
    FunctionalTestCase { id: 198, name: "gga_xc_th_fcfo" },
    FunctionalTestCase { id: 199, name: "gga_xc_th_fco" },
    FunctionalTestCase { id: 200, name: "gga_c_optc" },
    FunctionalTestCase { id: 215, name: "gga_x_ecmv92" },
    FunctionalTestCase { id: 216, name: "gga_c_pbe_vwn" },
    FunctionalTestCase { id: 217, name: "gga_c_p86_ft" },
    FunctionalTestCase { id: 218, name: "gga_k_rational_p" },
    FunctionalTestCase { id: 219, name: "gga_k_pg1" },
    FunctionalTestCase { id: 246, name: "gga_c_pbeloc" },
    FunctionalTestCase { id: 252, name: "gga_c_p86vwn" },
    FunctionalTestCase { id: 253, name: "gga_c_p86vwn_ft" },
    FunctionalTestCase { id: 255, name: "gga_xc_vv10" },
    FunctionalTestCase { id: 258, name: "gga_c_pbefe" },
    FunctionalTestCase { id: 262, name: "gga_c_op_pw91" },
    FunctionalTestCase { id: 265, name: "gga_x_pbefe" },
    FunctionalTestCase { id: 270, name: "gga_x_cap" },
    FunctionalTestCase { id: 271, name: "gga_x_eb88" },
    FunctionalTestCase { id: 272, name: "gga_c_pbe_mol" },
    FunctionalTestCase { id: 277, name: "gga_k_absp3" },
    FunctionalTestCase { id: 278, name: "gga_k_absp4" },
    FunctionalTestCase { id: 280, name: "gga_c_bmk" },
    FunctionalTestCase { id: 281, name: "gga_c_tau_hcth" },
    FunctionalTestCase { id: 283, name: "gga_c_hyb_tau_hcth" },
    FunctionalTestCase { id: 285, name: "gga_x_beefvdw" },
    FunctionalTestCase { id: 286, name: "gga_xc_beefvdw" },
    FunctionalTestCase { id: 291, name: "gga_x_pbetrans" },
    FunctionalTestCase { id: 298, name: "gga_x_chachiyo" },
    FunctionalTestCase { id: 309, name: "gga_c_chachiyo" },
    FunctionalTestCase { id: 312, name: "gga_x_revssb_d" },
    FunctionalTestCase { id: 313, name: "gga_c_ccdf" },
    FunctionalTestCase { id: 316, name: "gga_x_pw91_mod" },
    FunctionalTestCase { id: 320, name: "gga_x_pbe_mod" },
    FunctionalTestCase { id: 321, name: "gga_x_pbe_gaussian" },
    FunctionalTestCase { id: 322, name: "gga_c_pbe_gaussian" },
    FunctionalTestCase { id: 324, name: "gga_x_ncapr" },
    FunctionalTestCase { id: 327, name: "gga_xc_b97_3c" },
    FunctionalTestCase { id: 495, name: "gga_x_s12g" },
    FunctionalTestCase { id: 500, name: "gga_k_vw" },
    FunctionalTestCase { id: 501, name: "gga_k_ge2" },
    FunctionalTestCase { id: 502, name: "gga_k_golden" },
    FunctionalTestCase { id: 503, name: "gga_k_yt65" },
    FunctionalTestCase { id: 504, name: "gga_k_baltin" },
    FunctionalTestCase { id: 505, name: "gga_k_lieb" },
    FunctionalTestCase { id: 506, name: "gga_k_absp1" },
    FunctionalTestCase { id: 507, name: "gga_k_absp2" },
    FunctionalTestCase { id: 508, name: "gga_k_gr" },
    FunctionalTestCase { id: 509, name: "gga_k_ludena" },
    FunctionalTestCase { id: 510, name: "gga_k_gp85" },
    FunctionalTestCase { id: 511, name: "gga_k_pearson" },
    FunctionalTestCase { id: 512, name: "gga_k_ol1" },
    FunctionalTestCase { id: 513, name: "gga_k_ol2" },
    FunctionalTestCase { id: 514, name: "gga_k_fr_b88" },
    FunctionalTestCase { id: 515, name: "gga_k_fr_pw86" },
    FunctionalTestCase { id: 516, name: "gga_k_dk" },
    FunctionalTestCase { id: 517, name: "gga_k_perdew" },
    FunctionalTestCase { id: 518, name: "gga_k_vsk" },
    FunctionalTestCase { id: 519, name: "gga_k_vjks" },
    FunctionalTestCase { id: 520, name: "gga_k_ernzerhof" },
    FunctionalTestCase { id: 521, name: "gga_k_lc94" },
    FunctionalTestCase { id: 522, name: "gga_k_llp" },
    FunctionalTestCase { id: 523, name: "gga_k_thakkar" },
    FunctionalTestCase { id: 524, name: "gga_x_wpbeh" },
    FunctionalTestCase { id: 525, name: "gga_x_hjs_pbe" },
    FunctionalTestCase { id: 526, name: "gga_x_hjs_pbe_sol" },
    FunctionalTestCase { id: 527, name: "gga_x_hjs_b88" },
    FunctionalTestCase { id: 528, name: "gga_x_hjs_b97x" },
    FunctionalTestCase { id: 529, name: "gga_x_ityh" },
    FunctionalTestCase { id: 530, name: "gga_x_sfat" },
    FunctionalTestCase { id: 533, name: "gga_x_sg4" },
    FunctionalTestCase { id: 534, name: "gga_c_sg4" },
    FunctionalTestCase { id: 535, name: "gga_x_gg99" },
    FunctionalTestCase { id: 539, name: "gga_x_pbepow" },
    FunctionalTestCase { id: 544, name: "gga_x_kgg99" },
    FunctionalTestCase { id: 545, name: "gga_xc_hle16" },
    FunctionalTestCase { id: 553, name: "gga_c_scan_e0" },
    FunctionalTestCase { id: 555, name: "gga_c_gapc" },
    FunctionalTestCase { id: 556, name: "gga_c_gaploc" },
    FunctionalTestCase { id: 557, name: "gga_c_zvpbeint" },
    FunctionalTestCase { id: 558, name: "gga_c_zvpbesol" },
    FunctionalTestCase { id: 559, name: "gga_c_tm_lyp" },
    FunctionalTestCase { id: 560, name: "gga_c_tm_pbe" },
    FunctionalTestCase { id: 561, name: "gga_c_w94" },
    FunctionalTestCase { id: 565, name: "gga_c_cs1" },
    FunctionalTestCase { id: 570, name: "gga_x_b88m" },
    FunctionalTestCase { id: 587, name: "gga_xc_kt3" },
    FunctionalTestCase { id: 591, name: "gga_k_gds08" },
    FunctionalTestCase { id: 592, name: "gga_k_ghds10" },
    FunctionalTestCase { id: 593, name: "gga_k_ghds10r" },
    FunctionalTestCase { id: 594, name: "gga_k_tkvln" },
    FunctionalTestCase { id: 595, name: "gga_k_pbe3" },
    FunctionalTestCase { id: 596, name: "gga_k_pbe4" },
    FunctionalTestCase { id: 597, name: "gga_k_exp4" },
    FunctionalTestCase { id: 601, name: "gga_x_sfat_pbe" },
    FunctionalTestCase { id: 604, name: "gga_x_fd_lb94" },
    FunctionalTestCase { id: 605, name: "gga_x_fd_revlb94" },
    FunctionalTestCase { id: 606, name: "gga_c_zvpbeloc" },
    FunctionalTestCase { id: 613, name: "gga_k_lkt" },
    FunctionalTestCase { id: 616, name: "gga_k_pbe2" },
    FunctionalTestCase { id: 619, name: "gga_k_vt84f" },
    FunctionalTestCase { id: 620, name: "gga_k_lgap" },
    FunctionalTestCase { id: 622, name: "gga_x_ityh_optx" },
    FunctionalTestCase { id: 623, name: "gga_x_ityh_pbe" },
    FunctionalTestCase { id: 624, name: "gga_c_lypr" },
    FunctionalTestCase { id: 633, name: "gga_k_lgap_ge" },
    FunctionalTestCase { id: 635, name: "gga_k_tfvw_opt" },
    FunctionalTestCase { id: 655, name: "gga_x_pbe_erf_gws" },
    FunctionalTestCase { id: 657, name: "gga_c_pbe_erf_gws" },
    FunctionalTestCase { id: 712, name: "gga_c_mggac" },
    FunctionalTestCase { id: 734, name: "gga_x_q1d" },
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

// Test data: representative densities and reduced gradient (sigma) values
const RHO_UNPOL: &[f64] = &[0.1, 0.5, 1.0, 5.0];
const SIGMA_UNPOL: &[f64] = &[0.01, 0.1, 0.5, 2.0];

// Polarized: rho has 2 components per point, sigma has 3 (sigma_aa, sigma_ab, sigma_bb)
const RHO_POL: &[f64] = &[0.1, 0.05, 0.5, 0.3, 1.0, 0.8, 5.0, 3.0];
const SIGMA_POL: &[f64] = &[
    0.01, 0.005, 0.002,   // point 1
    0.1, 0.05, 0.02,      // point 2
    0.5, 0.2, 0.1,        // point 3
    2.0, 1.0, 0.5,        // point 4
];

/// Verify oracle calls succeed for all GGA functionals (unpolarized).
#[test]
fn test_all_gga_oracle_unpol() {
    let mut failures = Vec::new();
    let mut skipped = 0;
    for tc in GGA_FUNCTIONALS {
        let flags = oracle_func_flags(tc.id, 1).unwrap_or(0);
        if flags & FLAGS_HAVE_EXC == 0 {
            eprintln!("SKIP {}: no EXC support", tc.name);
            skipped += 1;
            continue;
        }
        match oracle_gga_all(tc.id, 1, RHO_UNPOL, SIGMA_UNPOL) {
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
            "GGA oracle unpolarized failures ({}/{}):\n  {}",
            failures.len(),
            GGA_FUNCTIONALS.len(),
            failures.join("\n  ")
        );
    }
    eprintln!(
        "GGA oracle unpolarized: {}/{} functionals OK ({} skipped, no EXC)",
        GGA_FUNCTIONALS.len() - skipped,
        GGA_FUNCTIONALS.len(),
        skipped,
    );
}

/// Verify oracle calls succeed for all GGA functionals (polarized).
#[test]
fn test_all_gga_oracle_pol() {
    let mut failures = Vec::new();
    let mut skipped = 0;
    for tc in GGA_FUNCTIONALS {
        let flags = oracle_func_flags(tc.id, 2).unwrap_or(0);
        if flags & FLAGS_HAVE_EXC == 0 {
            eprintln!("SKIP {}: no EXC support", tc.name);
            skipped += 1;
            continue;
        }
        match oracle_gga_all(tc.id, 2, RHO_POL, SIGMA_POL) {
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
            "GGA oracle polarized failures ({}/{}):\n  {}",
            failures.len(),
            GGA_FUNCTIONALS.len(),
            failures.join("\n  ")
        );
    }
    eprintln!(
        "GGA oracle polarized: {}/{} functionals OK ({} skipped, no EXC)",
        GGA_FUNCTIONALS.len() - skipped,
        GGA_FUNCTIONALS.len(),
        skipped,
    );
}
