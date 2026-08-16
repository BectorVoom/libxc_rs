//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1227/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1227(t40070: f64, t40076: f64, t40086: f64, t38056: f64, t38062: f64, t38617: f64, t40059: f64, t40064: f64, t40068: f64, t40073: f64, t40081: f64, t40084: f64) -> f64 {
    let t41680 = 0.11902492299418487743e0_f64 * t40070;
    let t41682 = 0.95219938395347901946e-2_f64 * t40076;
    let t41687 = 0.93149212406257582492e-1_f64 * t40086;
    let t41688 = 0.17336443480108537126e0_f64 * t40059 + 0.17465477326173296718e-1_f64 * t40064 + 0.52396431978519890152e-1_f64 * t40068 - t41680 + 0.2600466522016280569e0_f64 * t40073 + t41682 - t38617 - 0.23115257973478049502e0_f64 * t38056 + 0.93149212406257582492e-1_f64 * t38062 - 0.26198215989259945076e-1_f64 * t40081 - 0.1047928639570397803e0_f64 * t40084 - t41687;
    t41688
}
