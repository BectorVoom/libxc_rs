//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 991/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk991<F: Float>(t1444: F, t4605: F, t493: F, t834: F, t9354: F, t1972: F, t3243: F, t3247: F, t842: F, t3250: F, t5175: F, t5179: F, t5318: F, t1586: F, t1992: F, t5174: F) -> (F, F, F, F, F, F, F) {
    let t13477 = t1444 * t4605 / 15.0;
    let t13480 = t493 * t9354 * t834 / 45.0;
    let t13482 = t1972 * t3243 / 45.0;
    let t13483 = t3247 * t842;
    let t13486 = 8.0 / 81.0 * t493 * t13483 * t3250;
    let t13489 = 2.0 / 5.0 * t493 * t5179 * t5175;
    let t13492 = t493 * t5179 * t5318 / 5.0;
    let t13496 = t493 * t1992 * t5174 * t1586 / 5.0;
    (t13477, t13480, t13482, t13486, t13489, t13492, t13496)
}
