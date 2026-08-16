//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 813/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk813(t1284: f64, t5219: f64, t3624: f64, t12879: f64, t1715: f64, t247: f64, t1261: f64, t1803: f64, t3670: f64, t5436: f64, t1234: f64, t5390: f64) -> (f64, f64, f64, f64, f64) {
    let t17400 = t5219 * t1284;
    let t17401 = t17400 * t3624;
    let t17416 = t247 * t12879 * t1715;
    let t17417 = t1261 * t17416;
    let t17438 = t3670 * t1803;
    let t17448 = t5436 * t3624;
    let t17505 = t1234 * t5390;
    (t17401, t17417, t17438, t17448, t17505)
}
