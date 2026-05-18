//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1378/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1378<F: Float>(t26960: F, t26966: F, t28153: F, t92657: F, t93222: F, t96469: F, t96472: F, t96478: F, t96486: F, t96489: F, t96498: F, t96510: F, t96739: F, t96795: F, t96937: F) -> F {
    let t97487 = F::new(0.11607361111111111111e-2) * t96469 + F::new(0.19345601851851851852e-2) * t96472 - F::new(0.61905925925925925925e-2) * t96478 + F::new(0.17411041666666666666e-2) * t96486 - F::new(0.17411041666666666666e-2) * t96489 + F::new(0.34822083333333333332e-2) * t96498 - F::new(0.18534722222222222222e-2) * t26966 * t28153 + F::new(0.11349419753086419753e-1) * t96510 - F::new(0.46336805555555555556e-3) * t26960 * t96937 + F::new(0.30891203703703703704e-3) * t26960 * t96795 - F::new(0.61890573922526041667e-5) * t92657 * t96739 + F::new(0.15445601851851851852e-3) * t93222;
    t97487
}
