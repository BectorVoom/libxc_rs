//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1050/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1050(t11915: f64, t11918: f64, t15299: f64, t15947: f64, t176: f64, t1821: f64, t1826: f64, t1911: f64, t1912: f64, t1916: f64, t1920: f64, t1972: f64, t493: f64, t5486: f64, t6130: f64, t6134: f64, t6268: f64, t6398: f64, t6402: f64, t6407: f64, t6504: f64, t6747: f64) -> f64 {
    let t19595 = -t493 * t15947 * t1911 / 15.0_f64 - 2.0_f64 / 15.0_f64 * t493 * t6130 * t176 * t1826 + t493 * t15299 * t176 * t1821 / 9.0_f64 - t6134 * t1912 / 15.0_f64 - 2.0_f64 / 15.0_f64 * t6134 * t1916 + t6134 * t1920 / 9.0_f64 - t11915 - t11918 - 2.0_f64 / 3.0_f64 * t1972 * t6504 + 8.0_f64 / 15.0_f64 * t6268 * t6407 + 2.0_f64 / 15.0_f64 * t493 * t5486 * t6398 + 2.0_f64 / 5.0_f64 * t493 * t6747 * t6402;
    t19595
}
