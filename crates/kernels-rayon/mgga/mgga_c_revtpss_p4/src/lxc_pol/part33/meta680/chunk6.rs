//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2220/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2220(t5842: f64, t60: f64, t104379: f64, t108952: f64, t18281: f64, t1923: f64, t1927: f64, t19661: f64, t19666: f64, t19680: f64, t2123: f64, t26776: f64, t28089: f64, t28093: f64, t29355: f64, t29363: f64, t29364: f64, t29367: f64, t29372: f64, t29375: f64, t30682: f64, t30683: f64, t30686: f64, t4181: f64, t4186: f64, t606: f64, t6954: f64, t6977: f64, t72: f64, t7571: f64, t7702: f64, t7719: f64, t8143: f64, t8147: f64, t92612: f64, t96733: f64) -> f64 {
    let t111592 = t5842 * t60;
    let t111623 = -t108952 * t2123 / 6.0_f64 - t7702 * t29364 / 3.0_f64 - t7702 * t29367 / 3.0_f64 - t28093 * t8147 / 3.0_f64 - t7702 * t29372 / 3.0_f64 - t7702 * t29375 / 3.0_f64 - t6954 * t30683 / 6.0_f64 - t1923 * (-220.0_f64 / 27.0_f64 * t111592 * t606 - 40.0_f64 / 27.0_f64 * t104379 * t4181 + 40.0_f64 / 9.0_f64 * t29355 * t4186 + 5.0_f64 / 108.0_f64 * t96733 * t19661 + 5.0_f64 / 9.0_f64 * t26776 * t19666 + 5.0_f64 / 18.0_f64 * t26776 * t19680 - 5.0_f64 / 6.0_f64 * t7571 * t18281 + t92612) * t72 * t1927 / 6.0_f64 - t1923 * t30682 * t6977 / 6.0_f64 - t6954 * t30686 / 3.0_f64 - t1923 * t29363 * t7719 / 3.0_f64 - t1923 * t8143 * t28089 / 3.0_f64;
    t111623
}
