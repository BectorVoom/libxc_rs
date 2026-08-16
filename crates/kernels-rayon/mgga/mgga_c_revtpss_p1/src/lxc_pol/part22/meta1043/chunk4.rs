//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3650/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3650(t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64) -> f64 {
    let t69025 = -0.61805555555555555556e-2_f64 * t56230 - 0.19228395061728395062e-1_f64 * t56236 - 0.61805555555555555556e-2_f64 * t68389 + 0.92708333333333333333e-2_f64 * t68393 - 0.12361111111111111111e-1_f64 * t68397 + 0.82407407407407407409e-2_f64 * t68399 - 0.27469135802469135802e-2_f64 * t43865 - 0.19228395061728395061e-1_f64 * t43888 + 0.41203703703703703703e-2_f64 * t43890 + 0.82407407407407407406e-2_f64 * t43892 - 0.24722222222222222222e-1_f64 * t68454 - 0.37083333333333333334e-1_f64 * t68456 + 0.55625000000000000001e-1_f64 * t68459;
    t69025
}
