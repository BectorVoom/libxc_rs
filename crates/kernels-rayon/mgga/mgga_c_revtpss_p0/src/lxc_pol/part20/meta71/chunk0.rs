//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 463/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk463(t1455: f64, t117: f64, t670: f64, t572: f64, t573: f64, t76: f64, t84: f64, t198: f64, t207: f64, t159: f64, t215: f64, t10: f64, t17: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1459 = param_d * t1455;
    let t1461 = t117 * t670;
    let t1464 = t1459 * t573 + 3.0_f64 * t1461 * t572;
    let t1927 = t76 * t84;
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2219 = 2.0_f64 * t10 * t17;
    (t1459, t1461, t1464, t1927, t1940, t1941, t2219)
}
