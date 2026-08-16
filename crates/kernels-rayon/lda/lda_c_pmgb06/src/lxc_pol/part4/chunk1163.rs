//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1163/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1163(t1420: f64, t6297: f64, t2492: f64, t3216: f64, t439: f64, t6300: f64, t1454: f64, t493: f64, t6130: f64, t1461: f64, t2553: f64, t1466: f64) -> (f64, f64, f64, f64, f64) {
    let t15290 = 4.0_f64 / 45.0_f64 * t1420 * t6297;
    let t15293 = 2.0_f64 / 45.0_f64 * t439 * t3216 * t2492;
    let t15295 = 4.0_f64 / 45.0_f64 * t1420 * t6300;
    let t15298 = t493 * t6130 * t1454 / 45.0_f64;
    let t15299 = t1461 * t2553;
    let t15302 = t493 * t15299 * t1466 / 27.0_f64;
    (t15290, t15293, t15295, t15298, t15302)
}
