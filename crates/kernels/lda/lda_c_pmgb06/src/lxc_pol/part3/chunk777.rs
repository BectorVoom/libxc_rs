//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 777/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk777<F: Float>(t360: F, t8357: F, t1234: F, t409: F, t55: F, t3600: F, t1227: F, t1243: F, t3594: F, t110: F, t3589: F, t1263: F, t410: F, t1271: F, t1282: F, t8299: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8358 = t360 * t8357;
    let t8369 = t55 * t409 * t1234;
    let t8370 = t3600 * t8369;
    let t8373 = t55 * t409 * t1227;
    let t8374 = t1243 * t8373;
    let t8376 = t3594 * t8369;
    let t8378 = t110 * t3589;
    let t8379 = t360 * t8378;
    let t8381 = t410 * t1263;
    let t8382 = t360 * t8381;
    let t8386 = t1271 * t1282 * t97 * t8299;
    (t8358, t8370, t8373, t8374, t8376, t8378, t8379, t8381, t8382, t8386)
}
