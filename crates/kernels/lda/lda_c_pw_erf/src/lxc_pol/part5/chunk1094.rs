//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1094/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1094<F: Float>(t20355: F, t325: F, t431: F, t7927: F, t1: F, t2610: F, t322: F, t5592: F, t5607: F, t156: F, t426: F, t7897: F) -> (F, F, F, F, F) {
    let t20356 = F::cast_from(0.48717083333333333_f64) * t20355;
    let t20359 = t431 * t7927 * t325;
    let t20370 = t2610 * t1 * t322;
    let t20371 = t5592 * t20370;
    let t20373 = t5607 * t20370;
    let t20374 = F::cast_from(2.923025_f64) * t20373;
    let t20376 = t426 * t156 * t7897;
    (t20356, t20359, t20371, t20374, t20376)
}
