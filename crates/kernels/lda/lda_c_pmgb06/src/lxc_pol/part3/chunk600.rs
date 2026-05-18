//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 600/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk600<F: Float>(t1992: F, t3284: F, t493: F, t1444: F, t1455: F, t1592: F, t458: F, t1594: F, t137: F, t132: F, t1595: F, t435: F) -> (F, F, F, F, F, F, F, F) {
    let t3285 = t1992 * t3284;
    let t3287 = t493 * t3285 / F::new(5.0);
    let t3289 = t1444 * t1455 / F::new(15.0);
    let t3290 = t458 * t1592;
    let t3291 = t3290 * t1594;
    let t3292 = t137 * t3291;
    let t3294 = t132 * t3292 / F::new(5.0);
    let t3295 = t435 * t1595;
    (t3285, t3287, t3289, t3290, t3291, t3292, t3294, t3295)
}
