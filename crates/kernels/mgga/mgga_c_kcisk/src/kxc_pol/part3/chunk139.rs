//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 139/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk139<F: Float>(t416: F, t451: F, t140: F, t460: F, t476: F, t479: F, t467: F, sigma0: F) -> (F, F, F, F) {
    let t480 = t416 * t451;
    let t484 = 0.619125e-2 * t476 * t460 - 0.39796666666666666666e-1 * t140 * t479 * t480;
    let t485 = t484 * t467;
    let t486 = t485 * sigma0;
    (t480, t484, t485, t486)
}
