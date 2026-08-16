//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3610/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3610(t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64) -> f64 {
    let t68461 = -2.0_f64 / 9.0_f64 * t56230 - 56.0_f64 / 81.0_f64 * t56236 - 2.0_f64 / 9.0_f64 * t68389 + t68393 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t68397 + 8.0_f64 / 27.0_f64 * t68399 - 8.0_f64 / 81.0_f64 * t43865 - 56.0_f64 / 81.0_f64 * t43888 + 4.0_f64 / 27.0_f64 * t43890 + 8.0_f64 / 27.0_f64 * t43892 - 8.0_f64 / 9.0_f64 * t68454 - 4.0_f64 / 3.0_f64 * t68456 + 2.0_f64 * t68459;
    t68461
}
