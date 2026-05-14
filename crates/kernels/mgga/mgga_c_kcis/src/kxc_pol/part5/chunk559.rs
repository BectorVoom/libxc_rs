//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 559/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk559<F: Float>(t1035: F, t346: F, t1138: F, t1141: F, t1140: F, t392: F, t364: F, t1169: F, t284: F) -> (F, F, F, F, F) {
    let t3303 = t346 * t1035;
    let t3325 = t1138 * t1141;
    let t3329 = 1.0 / t1140 / t392;
    let t3330 = t364 * t3329;
    let t3337 = t1169 * t284;
    (t3303, t3325, t3329, t3330, t3337)
}
