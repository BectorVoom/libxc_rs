//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 799/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk799<F: Float>(t1080: F, t1414: F, t851: F, t1380: F, t493: F, t1423: F, t1894: F, t1594: F, t809: F, t2864: F, t439: F, t2022: F, t591: F) -> (F, F, F, F, F, F, F, F) {
    let t5358 = t851 * t1414 * t1080;
    let t5359 = t1380 * t5358;
    let t5361 = F::new(2.0) / F::new(45.0) * t493 * t5359;
    let t5363 = F::new(4.0) / F::new(135.0) * t1423 * t1894;
    let t5364 = t809 * t1594;
    let t5365 = t2864 * t5364;
    let t5367 = F::new(2.0) / F::new(45.0) * t439 * t5365;
    let t5369 = F::new(4.0) / F::new(9.0) * t2022 * t591;
    (t5358, t5359, t5361, t5363, t5364, t5365, t5367, t5369)
}
