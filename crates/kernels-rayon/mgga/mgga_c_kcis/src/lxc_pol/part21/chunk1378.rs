//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1378/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1378(t26960: f64, t26966: f64, t28153: f64, t92657: f64, t93222: f64, t96469: f64, t96472: f64, t96478: f64, t96486: f64, t96489: f64, t96498: f64, t96510: f64, t96739: f64, t96795: f64, t96937: f64) -> f64 {
    let t97487 = 0.11607361111111111111e-2_f64 * t96469 + 0.19345601851851851852e-2_f64 * t96472 - 0.61905925925925925925e-2_f64 * t96478 + 0.17411041666666666666e-2_f64 * t96486 - 0.17411041666666666666e-2_f64 * t96489 + 0.34822083333333333332e-2_f64 * t96498 - 0.18534722222222222222e-2_f64 * t26966 * t28153 + 0.11349419753086419753e-1_f64 * t96510 - 0.46336805555555555556e-3_f64 * t26960 * t96937 + 0.30891203703703703704e-3_f64 * t26960 * t96795 - 0.61890573922526041667e-5_f64 * t92657 * t96739 + 0.15445601851851851852e-3_f64 * t93222;
    t97487
}
