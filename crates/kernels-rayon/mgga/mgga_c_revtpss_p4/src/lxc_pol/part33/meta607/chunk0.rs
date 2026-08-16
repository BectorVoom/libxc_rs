//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2032/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2032(t2149: f64, t97312: f64, t1294: f64, t5464: f64, t1210: f64, t29199: f64, t1203: f64, t21471: f64, t3596: f64, t7627: f64, t26936: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97313 = t2149 * t97312;
    let t97314 = t5464 * t1294;
    let t97318 = t1210 * t29199;
    let t97319 = t21471 * t1203;
    let t97332 = t3596 * t7627;
    let t97343 = t3566 * t26936;
    (t97313, t97314, t97318, t97319, t97332, t97343)
}
