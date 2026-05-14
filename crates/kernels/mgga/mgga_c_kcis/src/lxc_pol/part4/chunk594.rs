//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 594/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk594<F: Float>(t3176: F, t3322: F, t393: F, t1138: F, t1141: F, t1203: F, t1140: F, t392: F, t364: F, t1171: F, t1175: F, t1170: F, t1169: F, t284: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3323 = t3176 + t3322;
    let t3324 = t3323 * t393;
    let t3325 = t1138 * t1141;
    let t3327 = 2.0 * t3325 * t1203;
    let t3329 = 1.0 / t1140 / t392;
    let t3330 = t364 * t3329;
    let t3331 = t1203 * t1203;
    let t3333 = 2.0 * t3330 * t3331;
    let t3334 = t1175 * t1171;
    let t3335 = t1170 * t3334;
    let t3337 = t1169 * t284;
    (t3323, t3324, t3325, t3327, t3329, t3330, t3331, t3333, t3334, t3335, t3337)
}
