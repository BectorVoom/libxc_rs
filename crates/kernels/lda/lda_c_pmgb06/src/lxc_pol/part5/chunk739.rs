//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 739/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk739<F: Float>(t132: F, t7503: F, t6423: F, t6425: F, t2549: F, t851: F, t1380: F, t493: F, t6759: F, t764: F, t1915: F, t6764: F, t1919: F, t2541: F, t2991: F, t6773: F, t760: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7505 = t132 * t7503 / 30.0;
    let t7506 = 4.0 / 45.0 * t6423;
    let t7507 = 4.0 / 45.0 * t6425;
    let t7508 = t2549 * t851;
    let t7509 = t1380 * t7508;
    let t7511 = t493 * t7509 / 15.0;
    let t7512 = t6759 * t764;
    let t7513 = t1915 * t7512;
    let t7515 = 2.0 / 15.0 * t493 * t7513;
    let t7516 = t6764 * t764;
    let t7517 = t1919 * t7516;
    let t7519 = t493 * t7517 / 9.0;
    let t7520 = t2541 * t851;
    let t7521 = t2991 * t7520;
    let t7523 = t493 * t7521 / 9.0;
    let t7524 = t6773 * t760;
    (t7505, t7506, t7507, t7508, t7509, t7511, t7512, t7513, t7515, t7516, t7517, t7519, t7520, t7521, t7523, t7524)
}
