//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1063/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1063(t1972: f64, t2974: f64, t1382: f64, t5194: f64, t1592: f64, t1962: f64, t2865: f64, t439: f64, t1602: f64, t1992: f64, t2088: f64, t3457: f64, t493: f64) -> (f64, f64, f64, f64) {
    let t12630 = 2.0_f64 / 15.0_f64 * t1972 * t2974;
    let t12631 = t5194 * t1382;
    let t12632 = 4.0_f64 / 45.0_f64 * t12631;
    let t12633 = t1962 * t1592;
    let t12636 = 2.0_f64 / 15.0_f64 * t439 * t12633 * t2865;
    let t12641 = 3.0_f64 / 5.0_f64 * t493 * t1992 * t3457 * t2088 * t1602;
    (t12630, t12632, t12636, t12641)
}
