//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 779/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk779<F: Float>(t69: F, t8315: F, t8381: F, t8378: F, t8357: F, t8312: F, t2247: F, t3650: F, t5858: F, t1289: F, t374: F, t6007: F, t342: F, t4232: F, t1311: F, t26: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8431 = t69 * t8315;
    let t8433 = t69 * t8381;
    let t8435 = t69 * t8378;
    let t8439 = t69 * t8357;
    let t8441 = t69 * t8312;
    let t8455 = t2247 * t5858 * t3650;
    let t8466 = t6007 * t1289 * t374;
    let t8470 = t4232 * t1289 * t342;
    let t8473 = t26 * t1311;
    (t8431, t8433, t8435, t8439, t8441, t8455, t8466, t8470, t8473)
}
