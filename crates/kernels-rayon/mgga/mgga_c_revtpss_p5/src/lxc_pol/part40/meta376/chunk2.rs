//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1339/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1339(t3022: f64, t4729: f64, t15399: f64, t15418: f64, t15420: f64, t15423: f64, t15425: f64, t15427: f64, t15477: f64, t15515: f64, t15549: f64, t15551: f64, t15553: f64, t15555: f64, t15558: f64, t15561: f64, t15571: f64, t15575: f64, t15577: f64) -> (f64, f64) {
    let t16181 = 0.11696447245269292414e1_f64 * t3022 * t4729;
    let t16182 = t15399 + t15418 + t15420 + t15423 + t15425 + t15427 + t15477 - t15549 - t15551 - t15553 - t15555 - t15558 - t15561 - t15515 + t15571 + t15575 + t15577 - t16181;
    (t16181, t16182)
}
