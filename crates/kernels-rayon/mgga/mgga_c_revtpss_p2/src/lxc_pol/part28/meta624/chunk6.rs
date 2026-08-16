//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2220/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2220(t4857: f64, t7131: f64, t16163: f64, t7122: f64, t15772: f64, t7132: f64, t15984: f64, t25517: f64, t1058: f64, t27464: f64, t3201: f64, t7801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100255 = t4857 * t7131;
    let t100261 = 0.57165357490759649296e-3_f64 * t7122 * t16163;
    let t100262 = t7132 * t15772;
    let t100268 = t25517 * t15984;
    let t100270 = t27464 * t1058;
    let t100272 = t7801 * t3201;
    (t100255, t100261, t100262, t100268, t100270, t100272)
}
