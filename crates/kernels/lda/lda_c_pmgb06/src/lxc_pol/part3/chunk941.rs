//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 941/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk941<F: Float>(t1414: F, t1601: F, t1602: F, t337: F, t764: F, t5068: F, t132: F, t137: F, t1395: F, t5039: F, t1083: F, t1380: F, t493: F, t5492: F, t1923: F, t2938: F) -> (F, F, F, F, F) {
    let t12691 = t1601 * t1414;
    let t12693 = t764 * t1602 * t337;
    let t12696 = 4.0 / 15.0 * t5068 * t12691 * t12693;
    let t12700 = t132 * t137 * t1395 * t5039 / 10.0;
    let t12704 = t493 * t1380 * t5492 * t1083 / 15.0;
    let t12708 = t493 * t1380 * t1923 * t2938 / 45.0;
    (t12693, t12696, t12700, t12704, t12708)
}
