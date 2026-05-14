//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1074/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1074<F: Float>(t16057: F, t1444: F, t6399: F, t2979: F, t493: F, t6398: F, t1380: F, t1586: F, t2545: F, t6403: F, t5447: F, t6402: F, t1083: F, t2541: F, t1915: F, t9402: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16058 = 8.0 / 27.0 * t16057;
    let t16060 = 4.0 / 45.0 * t1444 * t6399;
    let t16063 = 4.0 / 45.0 * t493 * t2979 * t6398;
    let t16067 = 2.0 / 45.0 * t493 * t1380 * t2545 * t1586;
    let t16069 = 4.0 / 15.0 * t1444 * t6403;
    let t16072 = 4.0 / 15.0 * t493 * t5447 * t6402;
    let t16073 = t2541 * t1083;
    let t16076 = 2.0 / 15.0 * t493 * t1915 * t16073;
    let t16077 = t9402 / 135.0;
    (t16058, t16060, t16063, t16067, t16069, t16072, t16073, t16076, t16077)
}
