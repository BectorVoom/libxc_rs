//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1225/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1225(t104115: f64, t1937: f64, t111734: f64, t29427: f64, t6993: f64, t125938: f64, t125942: f64, t125945: f64, t125948: f64, t125950: f64, t127299: f64, t127302: f64, t127305: f64, t127306: f64, t127308: f64) -> f64 {
    let t129414 = t104115 * t1937;
    let t129416 = t111734 * t1937;
    let t129418 = t29427 * t6993;
    let t129421 = t125938 + t125942 - 2.0_f64 * t125945 - t125948 - t125950 - 2.0_f64 * t129414 - 2.0_f64 * t129416 - 2.0_f64 * t129418 - t127299 + t127302 + t127305 - t127306 + 3.0_f64 * t127308;
    t129421
}
