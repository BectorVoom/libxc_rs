//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1163/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1163(t38: f64, t8142: f64, t2247: f64, t116: f64, t8151: f64, t1450: f64, t6816: f64, t7237: f64, t2014: f64, t6836: f64, t25864: f64, t1843: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29411 = t38 * t8142;
    let t29412 = t2247 * t29411;
    let t29427 = t8151 * t116;
    let t29494 = t1450 * t6816;
    let t29495 = t7237 * t29494;
    let t29497 = 3.0_f64 * t2014 * t29495;
    let t29498 = t1450 * t6836;
    let t29499 = t25864 * t29498;
    let t29501 = 6.0_f64 * t2014 * t29499;
    let t29502 = t1843 * t7741;
    (t29411, t29412, t29427, t29494, t29495, t29497, t29498, t29499, t29501, t29502)
}
