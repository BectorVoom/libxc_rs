//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 792/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk792(t1940: f64, t30: f64, t8490: f64, t8494: f64, t3140: f64, t3268: f64, t1078: f64, t1035: f64, t207: f64, t8489: f64, t8493: f64, t198: f64, t2411: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8498 = t1940 * t8490 * t30 / 2.0_f64 - t1940 * t8494 * t30 / 2.0_f64;
    let t8515 = t3140 * t3268;
    let t8520 = t3140 * t1078;
    let t8521 = t8520 * t1035;
    let t8536 = t207 * t8489;
    let t8539 = t207 * t8493;
    let t8542 = -t198 * t2411 * t8539 + t198 * t8536 * t892;
    (t8498, t8515, t8521, t8536, t8539, t8542)
}
