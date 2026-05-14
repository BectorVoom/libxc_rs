//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 903/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk903<F: Float>(t3393: F, t3407: F, t3166: F, t330: F, t3412: F, t1160: F, t318: F, t86: F, t1094: F, t3423: F, t284: F, t3473: F, t3177: F, t3436: F, t1194: F, t381: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t10558 = t3393 * t3407;
    let t10594 = t3166 * t330;
    let t10599 = t3393 * t3412;
    let t10631 = t86 * t318 * t1160;
    let t10691 = t3423 * t1094;
    let t10692 = t10691 * sigma0;
    let t10707 = t3473 * t284;
    let t10745 = t3177 * t3436;
    let t10752 = t381 * t1194;
    (t10558, t10594, t10599, t10631, t10692, t10707, t10745, t10752)
}
