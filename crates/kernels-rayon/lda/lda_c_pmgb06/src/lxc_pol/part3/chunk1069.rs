//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1069/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1069(t12691: f64, t12693: f64, t5068: f64, t132: f64, t137: f64, t1395: f64, t5039: f64, t1083: f64, t1380: f64, t493: f64, t5492: f64, t1923: f64, t2938: f64) -> (f64, f64, f64, f64) {
    let t12696 = 4.0_f64 / 15.0_f64 * t5068 * t12691 * t12693;
    let t12700 = t132 * t137 * t1395 * t5039 / 10.0_f64;
    let t12704 = t493 * t1380 * t5492 * t1083 / 15.0_f64;
    let t12708 = t493 * t1380 * t1923 * t2938 / 45.0_f64;
    (t12696, t12700, t12704, t12708)
}
