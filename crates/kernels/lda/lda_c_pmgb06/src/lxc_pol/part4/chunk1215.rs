//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1215/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1215<F: Float>(t1981: F, t2979: F, t6394: F, t1380: F, t493: F, t4935: F, t838: F, t15440: F, t1901: F, t439: F, t5168: F, t6382: F) -> (F, F, F, F) {
    let t16014 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1981 * t2979 * t6394;
    let t16018 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t1380 * t838 * t4935;
    let t16021 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t439 * t1901 * t15440;
    let t16023 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5168 * t6382;
    (t16014, t16018, t16021, t16023)
}
