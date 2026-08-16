//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 486/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk486<F: Float>(t1920: F, t493: F, t497: F, t851: F, t337: F, t1380: F) -> (F, F, F, F) {
    let t1922 = t493 * t1920 / F::cast_from(27.0_f64);
    let t1923 = t851 * t497;
    let t1924 = t1923 * t337;
    let t1925 = t1380 * t1924;
    (t1922, t1923, t1924, t1925)
}
