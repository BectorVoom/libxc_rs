//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1025/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1025(t5168: f64, t5248: f64, t2010: f64, t4668: f64, t5225: f64, t132: f64, t435: f64, t5119: f64, t3031: f64, t813: f64, t137: f64, t3033: f64) -> (f64, f64, f64, f64) {
    let t12186 = 8.0_f64 / 15.0_f64 * t5168 * t5248;
    let t12189 = 8.0_f64 / 15.0_f64 * t2010 * t5225 * t4668;
    let t12191 = t132 * t435 * t5119;
    let t12192 = 2.0_f64 / 15.0_f64 * t12191;
    let t12193 = t813 * t3031;
    let t12197 = t132 * t137 * t12193 * t3033 / 5.0_f64;
    (t12186, t12189, t12192, t12197)
}
