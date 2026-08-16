//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 717/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk717(t2073: f64, t4645: f64, t1324: f64, t2083: f64, t100: f64, t4577: f64, t1299: f64, t1329: f64, t2091: f64, t108: f64, t105: f64, t109: f64, t1327: f64, t1330: f64, t97: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4646 = t2073 * t4645;
    let t4649 = t1324 * t1324;
    let t4650 = t2083 * t4649;
    let t4653 = t100 * t4577;
    let t4656 = tau1 * t1299;
    let t4661 = t1329 * t1329;
    let t4662 = t2091 * t4661;
    let t4665 = -t4577;
    let t4666 = t108 * t4665;
    let t4669 = 10.0_f64 / 9.0_f64 * t97 * t4650 + 5.0_f64 / 3.0_f64 * t97 * t4653 + 40.0_f64 / 9.0_f64 * t4656 * t109 - 50.0_f64 / 9.0_f64 * t1327 * t1330 + 10.0_f64 / 9.0_f64 * t105 * t4662 + 5.0_f64 / 3.0_f64 * t105 * t4666;
    (t4646, t4649, t4650, t4653, t4656, t4661, t4665, t4669)
}
