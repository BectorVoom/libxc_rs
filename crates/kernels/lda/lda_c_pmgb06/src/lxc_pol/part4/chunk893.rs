//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 893/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk893<F: Float>(t1243: F, t8352: F, t1180: F, t361: F, t360: F, t1234: F, t409: F, t55: F, t3600: F, t1227: F, t3594: F, t1263: F, t410: F, t1271: F, t1282: F, t8299: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8355 = t1243 * t8352;
    let t8357 = t1180 * t361;
    let t8358 = t360 * t8357;
    let t8369 = t55 * t409 * t1234;
    let t8370 = t3600 * t8369;
    let t8373 = t55 * t409 * t1227;
    let t8374 = t1243 * t8373;
    let t8376 = t3594 * t8369;
    let t8381 = t410 * t1263;
    let t8382 = t360 * t8381;
    let t8386 = t1271 * t1282 * t97 * t8299;
    (t8355, t8357, t8358, t8370, t8373, t8374, t8376, t8381, t8382, t8386)
}
