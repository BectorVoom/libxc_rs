//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 812/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk812(t1786: f64, t3655: f64, t11262: f64, t1796: f64, t1247: f64, t1770: f64, t3140: f64, t3609: f64, t1802: f64, t474: f64, t3089: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17342 = t1786 * t3655;
    let t17361 = t11262 * t1796;
    let t17362 = t1247 * t17361;
    let t17376 = t1770 * t3140;
    let t17377 = t17376 * t3609;
    let t17394 = t474 * t1802;
    let t17395 = t17394 * t3089;
    let t17396 = t3717 * t17395;
    (t17342, t17362, t17376, t17377, t17395, t17396)
}
