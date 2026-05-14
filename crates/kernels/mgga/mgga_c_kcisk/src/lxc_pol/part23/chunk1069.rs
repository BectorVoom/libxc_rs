//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1069/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1069<F: Float>(t4513: F, t6591: F, t1591: F, t220: F, t4400: F, t6187: F, t18953: F, t4406: F, t1312: F, t3951: F, t964: F, t19087: F, t4391: F, t2059: F, t4497: F, t395: F, t397: F) -> (F, F, F, F, F, F, F, F) {
    let t21469 = t6591 * t4513;
    let t21478 = t220 * t1591;
    let t21479 = t4400 * t21478;
    let t21480 = t6187 * t21479;
    let t21483 = t4406 * t18953;
    let t21484 = t1312 * t21483;
    let t21487 = t964 * t3951;
    let t21488 = t4391 * t19087;
    let t21489 = t21487 * t21488;
    let t21492 = t2059 * t4497;
    let t21493 = t4400 * t21492;
    let t21494 = t1312 * t21493;
    let t21499 = t395 * t397;
    (t21469, t21478, t21480, t21484, t21489, t21492, t21494, t21499)
}
