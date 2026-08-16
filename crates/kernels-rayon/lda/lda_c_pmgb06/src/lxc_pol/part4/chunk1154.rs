//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1154/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1154(t15196: f64, t2002: f64, t5345: f64, t1080: f64, t6764: f64, t1915: f64, t493: f64, t10139: f64, t1602: f64, t2541: f64, t1447: f64, t6518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15197 = 8.0_f64 / 135.0_f64 * t15196;
    let t15199 = 4.0_f64 / 45.0_f64 * t2002 * t5345;
    let t15200 = t6764 * t1080;
    let t15203 = 2.0_f64 / 15.0_f64 * t493 * t1915 * t15200;
    let t15207 = 2.0_f64 / 27.0_f64 * t493 * t10139 * t2541 * t1602;
    let t15208 = t1447 * t6518;
    (t15197, t15199, t15200, t15203, t15207, t15208)
}
