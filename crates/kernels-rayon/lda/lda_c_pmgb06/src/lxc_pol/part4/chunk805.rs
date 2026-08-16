//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 805/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk805(t1380: f64, t5441: f64, t1981: f64, t1444: f64, t1916: f64, t1450: f64, t176: f64, t1826: f64, t493: f64, t1915: f64, t4847: f64, t1919: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5442 = t1380 * t5441;
    let t5444 = 4.0_f64 / 45.0_f64 * t1981 * t5442;
    let t5446 = 4.0_f64 / 45.0_f64 * t1444 * t1916;
    let t5447 = t1450 * t176;
    let t5448 = t5447 * t1826;
    let t5450 = 4.0_f64 / 45.0_f64 * t493 * t5448;
    let t5451 = t1915 * t4847;
    let t5453 = 2.0_f64 / 45.0_f64 * t493 * t5451;
    let t5454 = t1919 * t4857;
    (t5442, t5444, t5446, t5447, t5448, t5450, t5451, t5453, t5454)
}
