//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 624/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk624<F: Float>(t144: F, t3259: F, t1423: F, t1431: F, t1441: F, t1435: F, t458: F, t1592: F, t1595: F, t435: F, t132: F, t1555: F, t486: F, t186: F, t409: F, t55: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3260 = t3259 * t144;
    let t3272 = t1423 * t1431;
    let t3274 = t1423 * t1441;
    let t3279 = t1435 * t458;
    let t3290 = t458 * t1592;
    let t3295 = t435 * t1595;
    let t3296 = t132 * t3295;
    let t3306 = t486 * t1555;
    let t3309 = t55 * t409 * t186;
    (t3260, t3272, t3274, t3279, t3290, t3295, t3296, t3306, t3309)
}
