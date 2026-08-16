//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1094/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1094(t20355: f64, t325: f64, t431: f64, t7927: f64, t1: f64, t2610: f64, t322: f64, t5592: f64, t5607: f64, t156: f64, t426: f64, t7897: f64) -> (f64, f64, f64, f64, f64) {
    let t20356 = 0.48717083333333333_f64 * t20355;
    let t20359 = t431 * t7927 * t325;
    let t20370 = t2610 * t1 * t322;
    let t20371 = t5592 * t20370;
    let t20373 = t5607 * t20370;
    let t20374 = 2.923025_f64 * t20373;
    let t20376 = t426 * t156 * t7897;
    (t20356, t20359, t20371, t20374, t20376)
}
