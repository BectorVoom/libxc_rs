//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1430/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1430(t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41329: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t11852: f64, t159: f64) -> (f64, f64) {
    let t41338 = 8.0_f64 / 3.0_f64 * t41308 + 8.0_f64 * t41312 - 12.0_f64 * t41316 + 2.0_f64 * t41320 + 8.0_f64 * t41323 - 2.0_f64 / 3.0_f64 * t41327 + t41329 - 8.0_f64 / 9.0_f64 * t41330 - 16.0_f64 / 27.0_f64 * t41332 + 4.0_f64 / 9.0_f64 * t41334 + 40.0_f64 / 81.0_f64 * t41336;
    let t41339 = t159 * t11852;
    (t41338, t41339)
}
