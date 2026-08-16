//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1536/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1536(t16272: f64, t16310: f64, t16355: f64, t16610: f64, t1100: f64, t1102: f64, t15418: f64, t15420: f64, t15423: f64, t15425: f64, t15427: f64, t15477: f64, t15515: f64, t15549: f64, t15551: f64, t15553: f64, t15555: f64, t15558: f64, t15561: f64, t15562: f64, t15566: f64, t15571: f64, t15575: f64, t15577: f64, t16181: f64, t198: f64, t3333: f64, t336: f64, t5023: f64) -> f64 {
    let t16612 = t16272 + t16310 + t16355 + t16610;
    let t16616 = t1102 * t16612 * t198 * t336 - 2.0_f64 * t1100 * t15562 * t5023 + 2.0_f64 * t15566 * t3333 * t5023 + t15418 + t15420 + t15423 + t15425 + t15427 + t15477 - t15515 - t15549 - t15551 - t15553 - t15555 - t15558 - t15561 + t15571 + t15575 + t15577 - t16181;
    t16616
}
