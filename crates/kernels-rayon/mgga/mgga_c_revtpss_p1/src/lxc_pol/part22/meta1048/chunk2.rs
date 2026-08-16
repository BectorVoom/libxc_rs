//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3684/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3684(t12050: f64, t20956: f64, t1261: f64, t12879: f64, t247: f64, t6429: f64, t11262: f64, t1247: f64, t6624: f64, t21102: f64, t3704: f64, t17376: f64, t17524: f64) -> (f64, f64, f64, f64, f64) {
    let t69655 = t20956 * t12050;
    let t69661 = t1261 * t247 * t12879 * t6429;
    let t69668 = t1247 * t11262 * t6624;
    let t69674 = t21102 * t3704;
    let t69680 = t17376 * t17524;
    (t69655, t69661, t69668, t69674, t69680)
}
