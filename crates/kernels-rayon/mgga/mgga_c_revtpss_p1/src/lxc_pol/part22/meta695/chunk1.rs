//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2701/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2701(t13633: f64, t13615: f64, t13620: f64, t13623: f64, t13634: f64, t13635: f64, t22192: f64, t22194: f64, t22196: f64, t22197: f64, t22198: f64, t22199: f64, t22200: f64, t22201: f64, t9394: f64, t9415: f64) -> (f64, f64) {
    let t22202 = 2.0_f64 * t13633;
    let t22203 = -t22192 + t22194 + t22196 - t22197 - t13615 + t9394 - t13620 - t22198 - t13623 - t22199 - t22200 + t22201 + t22202 + t13634 - t13635 - t9415;
    (t22202, t22203)
}
