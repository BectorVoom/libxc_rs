//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 850/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk850(t140: f64, t6652: f64, t1222: f64, t1234: f64, t6594: f64, t3172: f64, t6630: f64, t3600: f64, t247: f64, t3634: f64, t6425: f64, t1261: f64) -> (f64, f64, f64, f64) {
    let t21169 = t140 * t6652;
    let t21170 = t1222 * t21169;
    let t21177 = t1234 * t6594;
    let t21188 = t3172 * t6630;
    let t21189 = t3600 * t21188;
    let t21192 = t247 * t3634 * t6425;
    let t21193 = t1261 * t21192;
    (t21170, t21177, t21189, t21193)
}
