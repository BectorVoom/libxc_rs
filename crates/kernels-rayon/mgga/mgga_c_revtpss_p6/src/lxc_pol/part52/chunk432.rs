//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 432/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk432(t2113: f64, t2115: f64, t572: f64, t573: f64, t10: f64, t17: f64, t15: f64, t22: f64, t11: f64, t14: f64, t20: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t2118 = t2113 * t573 + 3.0_f64 * t2115 * t572;
    let t2219 = 2.0_f64 * t10 * t17;
    let t2223 = 6.0_f64 * t15 * t22;
    let t2224 = t11 * t14;
    let t2226 = 12.0_f64 * t2224 * t22;
    let t2230 = 20.0_f64 * t20 * t27;
    (t2118, t2219, t2223, t2226, t2230)
}
