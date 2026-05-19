//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1378/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1378<F: Float>(t26960: F, t26966: F, t28153: F, t92657: F, t93222: F, t96469: F, t96472: F, t96478: F, t96486: F, t96489: F, t96498: F, t96510: F, t96739: F, t96795: F, t96937: F) -> F {
    let t97487 = F::cast_from(0.11607361111111111111e-2_f64) * t96469 + F::cast_from(0.19345601851851851852e-2_f64) * t96472 - F::cast_from(0.61905925925925925925e-2_f64) * t96478 + F::cast_from(0.17411041666666666666e-2_f64) * t96486 - F::cast_from(0.17411041666666666666e-2_f64) * t96489 + F::cast_from(0.34822083333333333332e-2_f64) * t96498 - F::cast_from(0.18534722222222222222e-2_f64) * t26966 * t28153 + F::cast_from(0.11349419753086419753e-1_f64) * t96510 - F::cast_from(0.46336805555555555556e-3_f64) * t26960 * t96937 + F::cast_from(0.30891203703703703704e-3_f64) * t26960 * t96795 - F::cast_from(0.61890573922526041667e-5_f64) * t92657 * t96739 + F::cast_from(0.15445601851851851852e-3_f64) * t93222;
    t97487
}
