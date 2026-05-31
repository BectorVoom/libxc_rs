//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1069/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1069<F: Float>(t12691: F, t12693: F, t5068: F, t132: F, t137: F, t1395: F, t5039: F, t1083: F, t1380: F, t493: F, t5492: F, t1923: F, t2938: F) -> (F, F, F, F) {
    let t12696 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5068 * t12691 * t12693;
    let t12700 = t132 * t137 * t1395 * t5039 / F::cast_from(10.0_f64);
    let t12704 = t493 * t1380 * t5492 * t1083 / F::cast_from(15.0_f64);
    let t12708 = t493 * t1380 * t1923 * t2938 / F::cast_from(45.0_f64);
    (t12696, t12700, t12704, t12708)
}
