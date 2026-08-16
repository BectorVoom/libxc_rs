//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2066/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2066(t3596: f64, t37885: f64, t2149: f64, t1210: f64, t29199: f64, t26936: f64, t3566: f64, t13181: f64, t3140: f64, t1243: f64, t2147: f64, t44841: f64, t7635: f64) -> (f64, f64, f64, f64, f64) {
    let t97312 = t37885 * t3596;
    let t97313 = t2149 * t97312;
    let t97318 = t1210 * t29199;
    let t97343 = t3566 * t26936;
    let t97346 = t3140 * t13181;
    let t97348 = t2149 * t97346 * t1243;
    let t97358 = t2147 * t44841 * t7635;
    (t97313, t97318, t97343, t97348, t97358)
}
