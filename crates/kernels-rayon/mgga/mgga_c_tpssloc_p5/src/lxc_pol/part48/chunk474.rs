//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 474/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk474(t475: f64, t3507: f64, t1214: f64, t248: f64, t1210: f64, t3504: f64, t3500: f64, t121: f64, t1229: f64, t1090: f64, t1227: f64, t1230: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3508 = t475 * t475;
    let t3509 = t3507 * t3508;
    let t3511 = t248 * t1214 * t3509;
    let t3514 = t1210 * t3504;
    let t3515 = t3500 * t3514;
    let t3516 = t3507 * t475;
    let t3518 = t248 * t1214 * t3516;
    let t3521 = t121 * t1229;
    let t3523 = t248 * t3521 * t1090;
    let t3524 = t1227 * t3523;
    let t3527 = t248 * t1230 * t3252;
    (t3508, t3511, t3515, t3518, t3523, t3524, t3527)
}
