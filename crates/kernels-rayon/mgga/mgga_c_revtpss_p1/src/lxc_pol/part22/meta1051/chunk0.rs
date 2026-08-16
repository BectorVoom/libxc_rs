//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3705/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3705(t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64) -> f64 {
    let t70200 = -0.9877777777777777778e-2_f64 * t56230 - 0.30730864197530864199e-1_f64 * t56236 - 0.9877777777777777778e-2_f64 * t68389 + 0.14816666666666666667e-1_f64 * t68393 - 0.19755555555555555556e-1_f64 * t68397 + 0.13170370370370370371e-1_f64 * t68399 - 0.43901234567901234569e-2_f64 * t43865 - 0.30730864197530864198e-1_f64 * t43888 + 0.65851851851851851853e-2_f64 * t43890 + 0.13170370370370370371e-1_f64 * t43892 - 0.39511111111111111112e-1_f64 * t68454 - 0.59266666666666666668e-1_f64 * t68456 + 0.88900000000000000002e-1_f64 * t68459;
    t70200
}
