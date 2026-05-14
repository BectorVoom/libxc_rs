//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 574/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk574<F: Float>(t1592: F, t458: F, t1594: F, t137: F, t132: F, t1595: F, t435: F, t1596: F, t432: F, t1919: F, t2924: F, t493: F, t1901: F, t3104: F, t439: F, t1555: F, t486: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3290 = t458 * t1592;
    let t3291 = t3290 * t1594;
    let t3292 = t137 * t3291;
    let t3294 = t132 * t3292 / 5.0;
    let t3295 = t435 * t1595;
    let t3296 = t132 * t3295;
    let t3297 = 2.0 / 15.0 * t3296;
    let t3299 = t432 * t1596 / 5.0;
    let t3300 = t1919 * t2924;
    let t3302 = t493 * t3300 / 9.0;
    let t3303 = t1901 * t3104;
    let t3305 = t439 * t3303 / 9.0;
    let t3306 = t486 * t1555;
    (t3290, t3291, t3292, t3294, t3295, t3296, t3297, t3299, t3300, t3302, t3303, t3305, t3306)
}
