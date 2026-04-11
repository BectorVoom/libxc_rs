//! Batch oracle verification for all MGGA functionals.
//!
//! Verifies that the C libxc oracle can successfully evaluate all MGGA functionals.
//! Per-functional Rust-vs-oracle comparison tests will be activated as kernels are translated.

use libxc_rs_verify::{oracle_mgga_all, oracle_func_flags, FLAGS_HAVE_EXC};

struct FunctionalTestCase {
    id: i32,
    name: &'static str,
}

/// All 146 MGGA functionals from libxc 7.0.0 (xc_funcs.h).
const MGGA_FUNCTIONALS: &[FunctionalTestCase] = &[
    FunctionalTestCase { id: 37, name: "mgga_c_dldf" },
    FunctionalTestCase { id: 42, name: "mgga_xc_zlp" },
    FunctionalTestCase { id: 64, name: "mgga_xc_otpss_d" },
    FunctionalTestCase { id: 72, name: "mgga_c_cs" },
    FunctionalTestCase { id: 73, name: "mgga_c_mn12_sx" },
    FunctionalTestCase { id: 74, name: "mgga_c_mn12_l" },
    FunctionalTestCase { id: 75, name: "mgga_c_m11_l" },
    FunctionalTestCase { id: 76, name: "mgga_c_m11" },
    FunctionalTestCase { id: 77, name: "mgga_c_m08_so" },
    FunctionalTestCase { id: 78, name: "mgga_c_m08_hx" },
    FunctionalTestCase { id: 172, name: "mgga_c_revm11" },
    FunctionalTestCase { id: 201, name: "mgga_x_lta" },
    FunctionalTestCase { id: 202, name: "mgga_x_tpss" },
    FunctionalTestCase { id: 203, name: "mgga_x_m06_l" },
    FunctionalTestCase { id: 204, name: "mgga_x_gvt4" },
    FunctionalTestCase { id: 205, name: "mgga_x_tau_hcth" },
    FunctionalTestCase { id: 206, name: "mgga_x_br89" },
    FunctionalTestCase { id: 207, name: "mgga_x_bj06" },
    FunctionalTestCase { id: 208, name: "mgga_x_tb09" },
    FunctionalTestCase { id: 209, name: "mgga_x_rpp09" },
    FunctionalTestCase { id: 210, name: "mgga_x_2d_prhg07" },
    FunctionalTestCase { id: 211, name: "mgga_x_2d_prhg07_prp10" },
    FunctionalTestCase { id: 212, name: "mgga_x_revtpss" },
    FunctionalTestCase { id: 213, name: "mgga_x_pkzb" },
    FunctionalTestCase { id: 214, name: "mgga_x_br89_1" },
    FunctionalTestCase { id: 220, name: "mgga_k_pgsl025" },
    FunctionalTestCase { id: 221, name: "mgga_x_ms0" },
    FunctionalTestCase { id: 222, name: "mgga_x_ms1" },
    FunctionalTestCase { id: 223, name: "mgga_x_ms2" },
    FunctionalTestCase { id: 225, name: "mgga_x_th" },
    FunctionalTestCase { id: 226, name: "mgga_x_m11_l" },
    FunctionalTestCase { id: 227, name: "mgga_x_mn12_l" },
    FunctionalTestCase { id: 228, name: "mgga_x_ms2_rev" },
    FunctionalTestCase { id: 229, name: "mgga_xc_cc06" },
    FunctionalTestCase { id: 230, name: "mgga_x_mk00" },
    FunctionalTestCase { id: 231, name: "mgga_c_tpss" },
    FunctionalTestCase { id: 232, name: "mgga_c_vsxc" },
    FunctionalTestCase { id: 233, name: "mgga_c_m06_l" },
    FunctionalTestCase { id: 234, name: "mgga_c_m06_hf" },
    FunctionalTestCase { id: 235, name: "mgga_c_m06" },
    FunctionalTestCase { id: 236, name: "mgga_c_m06_2x" },
    FunctionalTestCase { id: 237, name: "mgga_c_m05" },
    FunctionalTestCase { id: 238, name: "mgga_c_m05_2x" },
    FunctionalTestCase { id: 239, name: "mgga_c_pkzb" },
    FunctionalTestCase { id: 240, name: "mgga_c_bc95" },
    FunctionalTestCase { id: 241, name: "mgga_c_revtpss" },
    FunctionalTestCase { id: 242, name: "mgga_xc_tpsslyp1w" },
    FunctionalTestCase { id: 243, name: "mgga_x_mk00b" },
    FunctionalTestCase { id: 244, name: "mgga_x_bloc" },
    FunctionalTestCase { id: 245, name: "mgga_x_modtpss" },
    FunctionalTestCase { id: 247, name: "mgga_c_tpssloc" },
    FunctionalTestCase { id: 249, name: "mgga_x_mbeef" },
    FunctionalTestCase { id: 250, name: "mgga_x_mbeefvdw" },
    FunctionalTestCase { id: 251, name: "mgga_c_tm" },
    FunctionalTestCase { id: 254, name: "mgga_xc_b97m_v" },
    FunctionalTestCase { id: 256, name: "mgga_x_jk" },
    FunctionalTestCase { id: 257, name: "mgga_x_mvs" },
    FunctionalTestCase { id: 260, name: "mgga_x_mn15_l" },
    FunctionalTestCase { id: 261, name: "mgga_c_mn15_l" },
    FunctionalTestCase { id: 263, name: "mgga_x_scan" },
    FunctionalTestCase { id: 267, name: "mgga_c_scan" },
    FunctionalTestCase { id: 269, name: "mgga_c_mn15" },
    FunctionalTestCase { id: 284, name: "mgga_x_b00" },
    FunctionalTestCase { id: 288, name: "mgga_xc_hle17" },
    FunctionalTestCase { id: 292, name: "mgga_c_scan_rvv10" },
    FunctionalTestCase { id: 293, name: "mgga_x_revm06_l" },
    FunctionalTestCase { id: 294, name: "mgga_c_revm06_l" },
    FunctionalTestCase { id: 299, name: "mgga_x_rtpss" },
    FunctionalTestCase { id: 300, name: "mgga_x_ms2b" },
    FunctionalTestCase { id: 301, name: "mgga_x_ms2bs" },
    FunctionalTestCase { id: 302, name: "mgga_x_mvsb" },
    FunctionalTestCase { id: 303, name: "mgga_x_mvsbs" },
    FunctionalTestCase { id: 306, name: "mgga_c_revm06" },
    FunctionalTestCase { id: 311, name: "mgga_c_m06_sx" },
    FunctionalTestCase { id: 319, name: "mgga_x_ft98" },
    FunctionalTestCase { id: 323, name: "mgga_c_tpss_gaussian" },
    FunctionalTestCase { id: 387, name: "mgga_c_cc" },
    FunctionalTestCase { id: 388, name: "mgga_c_ccalda" },
    FunctionalTestCase { id: 391, name: "mgga_c_rregtm" },
    FunctionalTestCase { id: 397, name: "mgga_c_b94" },
    FunctionalTestCase { id: 493, name: "mgga_x_rscan" },
    FunctionalTestCase { id: 494, name: "mgga_c_rscan" },
    FunctionalTestCase { id: 497, name: "mgga_x_r2scan" },
    FunctionalTestCase { id: 498, name: "mgga_c_r2scan" },
    FunctionalTestCase { id: 540, name: "mgga_x_tm" },
    FunctionalTestCase { id: 541, name: "mgga_x_vt84" },
    FunctionalTestCase { id: 542, name: "mgga_x_sa_tpss" },
    FunctionalTestCase { id: 543, name: "mgga_k_pc07" },
    FunctionalTestCase { id: 562, name: "mgga_c_kcis" },
    FunctionalTestCase { id: 564, name: "mgga_xc_lp90" },
    FunctionalTestCase { id: 571, name: "mgga_c_b88" },
    FunctionalTestCase { id: 575, name: "mgga_x_gx" },
    FunctionalTestCase { id: 576, name: "mgga_x_pbe_gx" },
    FunctionalTestCase { id: 581, name: "mgga_x_revscan" },
    FunctionalTestCase { id: 582, name: "mgga_c_revscan" },
    FunctionalTestCase { id: 584, name: "mgga_c_scan_vv10" },
    FunctionalTestCase { id: 585, name: "mgga_c_revscan_vv10" },
    FunctionalTestCase { id: 586, name: "mgga_x_br89_explicit" },
    FunctionalTestCase { id: 602, name: "mgga_x_br89_explicit_1" },
    FunctionalTestCase { id: 603, name: "mgga_x_regtpss" },
    FunctionalTestCase { id: 609, name: "mgga_x_2d_js17" },
    FunctionalTestCase { id: 617, name: "mgga_k_l04" },
    FunctionalTestCase { id: 618, name: "mgga_k_l06" },
    FunctionalTestCase { id: 621, name: "mgga_k_rda" },
    FunctionalTestCase { id: 626, name: "mgga_x_regtm" },
    FunctionalTestCase { id: 627, name: "mgga_k_gea2" },
    FunctionalTestCase { id: 628, name: "mgga_k_gea4" },
    FunctionalTestCase { id: 629, name: "mgga_k_csk1" },
    FunctionalTestCase { id: 630, name: "mgga_k_csk4" },
    FunctionalTestCase { id: 631, name: "mgga_k_csk_loc1" },
    FunctionalTestCase { id: 632, name: "mgga_k_csk_loc4" },
    FunctionalTestCase { id: 634, name: "mgga_k_pc07_opt" },
    FunctionalTestCase { id: 638, name: "mgga_c_kcisk" },
    FunctionalTestCase { id: 642, name: "mgga_c_r2scan01" },
    FunctionalTestCase { id: 643, name: "mgga_c_rmggac" },
    FunctionalTestCase { id: 644, name: "mgga_x_mcml" },
    FunctionalTestCase { id: 645, name: "mgga_x_r2scan01" },
    FunctionalTestCase { id: 648, name: "mgga_x_rppscan" },
    FunctionalTestCase { id: 649, name: "mgga_c_rppscan" },
    FunctionalTestCase { id: 650, name: "mgga_x_r4scan" },
    FunctionalTestCase { id: 651, name: "mgga_x_vcml" },
    FunctionalTestCase { id: 652, name: "mgga_xc_vcml_rvv10" },
    FunctionalTestCase { id: 685, name: "mgga_x_tlda" },
    FunctionalTestCase { id: 686, name: "mgga_x_edmgga" },
    FunctionalTestCase { id: 687, name: "mgga_x_gdme_nv" },
    FunctionalTestCase { id: 688, name: "mgga_x_rlda" },
    FunctionalTestCase { id: 689, name: "mgga_x_gdme_0" },
    FunctionalTestCase { id: 690, name: "mgga_x_gdme_kos" },
    FunctionalTestCase { id: 691, name: "mgga_x_gdme_vt" },
    FunctionalTestCase { id: 693, name: "mgga_x_revtm" },
    FunctionalTestCase { id: 694, name: "mgga_c_revtm" },
    FunctionalTestCase { id: 696, name: "mgga_x_mbrxc_bg" },
    FunctionalTestCase { id: 697, name: "mgga_x_mbrxh_bg" },
    FunctionalTestCase { id: 698, name: "mgga_x_hlta" },
    FunctionalTestCase { id: 699, name: "mgga_c_hltapw" },
    FunctionalTestCase { id: 700, name: "mgga_x_scanl" },
    FunctionalTestCase { id: 701, name: "mgga_x_revscanl" },
    FunctionalTestCase { id: 702, name: "mgga_c_scanl" },
    FunctionalTestCase { id: 703, name: "mgga_c_scanl_rvv10" },
    FunctionalTestCase { id: 704, name: "mgga_c_scanl_vv10" },
    FunctionalTestCase { id: 707, name: "mgga_x_task" },
    FunctionalTestCase { id: 711, name: "mgga_x_mggac" },
    FunctionalTestCase { id: 716, name: "mgga_x_mbr" },
    FunctionalTestCase { id: 718, name: "mgga_x_r2scanl" },
    FunctionalTestCase { id: 719, name: "mgga_c_r2scanl" },
    FunctionalTestCase { id: 724, name: "mgga_x_mtask" },
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

// Test data: representative densities and MGGA inputs
const RHO_UNPOL: &[f64] = &[0.1, 0.5, 1.0, 5.0];
const SIGMA_UNPOL: &[f64] = &[0.01, 0.1, 0.5, 2.0];
const LAPL_UNPOL: &[f64] = &[0.001, 0.01, 0.05, 0.2];
const TAU_UNPOL: &[f64] = &[0.01, 0.05, 0.2, 1.0];

// Polarized: rho=2, sigma=3, lapl=2, tau=2 components per point
const RHO_POL: &[f64] = &[0.1, 0.05, 0.5, 0.3, 1.0, 0.8, 5.0, 3.0];
const SIGMA_POL: &[f64] = &[
    0.01, 0.005, 0.002,   // point 1
    0.1, 0.05, 0.02,      // point 2
    0.5, 0.2, 0.1,        // point 3
    2.0, 1.0, 0.5,        // point 4
];
const LAPL_POL: &[f64] = &[
    0.001, 0.0005,   // point 1
    0.01, 0.005,     // point 2
    0.05, 0.02,      // point 3
    0.2, 0.1,        // point 4
];
const TAU_POL: &[f64] = &[
    0.01, 0.005,     // point 1
    0.05, 0.03,      // point 2
    0.2, 0.1,        // point 3
    1.0, 0.5,        // point 4
];

/// Verify oracle calls succeed for all MGGA functionals (unpolarized).
#[test]
fn test_all_mgga_oracle_unpol() {
    let mut failures = Vec::new();
    let mut skipped = 0;
    for tc in MGGA_FUNCTIONALS {
        let flags = oracle_func_flags(tc.id, 1).unwrap_or(0);
        if flags & FLAGS_HAVE_EXC == 0 {
            eprintln!("SKIP {}: no EXC support", tc.name);
            skipped += 1;
            continue;
        }
        match oracle_mgga_all(tc.id, 1, RHO_UNPOL, SIGMA_UNPOL, LAPL_UNPOL, TAU_UNPOL) {
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
            "MGGA oracle unpolarized failures ({}/{}):\n  {}",
            failures.len(),
            MGGA_FUNCTIONALS.len(),
            failures.join("\n  ")
        );
    }
    eprintln!(
        "MGGA oracle unpolarized: {}/{} functionals OK ({} skipped, no EXC)",
        MGGA_FUNCTIONALS.len() - skipped,
        MGGA_FUNCTIONALS.len(),
        skipped,
    );
}

/// Verify oracle calls succeed for all MGGA functionals (polarized).
#[test]
fn test_all_mgga_oracle_pol() {
    let mut failures = Vec::new();
    let mut skipped = 0;
    for tc in MGGA_FUNCTIONALS {
        let flags = oracle_func_flags(tc.id, 2).unwrap_or(0);
        if flags & FLAGS_HAVE_EXC == 0 {
            eprintln!("SKIP {}: no EXC support", tc.name);
            skipped += 1;
            continue;
        }
        match oracle_mgga_all(tc.id, 2, RHO_POL, SIGMA_POL, LAPL_POL, TAU_POL) {
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
            "MGGA oracle polarized failures ({}/{}):\n  {}",
            failures.len(),
            MGGA_FUNCTIONALS.len(),
            failures.join("\n  ")
        );
    }
    eprintln!(
        "MGGA oracle polarized: {}/{} functionals OK ({} skipped, no EXC)",
        MGGA_FUNCTIONALS.len() - skipped,
        MGGA_FUNCTIONALS.len(),
        skipped,
    );
}
