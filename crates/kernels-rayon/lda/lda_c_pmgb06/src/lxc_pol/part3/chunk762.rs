//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 762/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk762(t2012: f64, t5168: f64, t3216: f64, t805: f64, t439: f64, t1600: f64, t2088: f64, t529: f64, t1992: f64, t493: f64, t165: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5170 = 4.0_f64 / 45.0_f64 * t5168 * t2012;
    let t5171 = t3216 * t805;
    let t5173 = t439 * t5171 / 45.0_f64;
    let t5174 = t1600 * t2088;
    let t5175 = t5174 * t529;
    let t5176 = t1992 * t5175;
    let t5178 = 2.0_f64 / 15.0_f64 * t493 * t5176;
    let t5179 = t165 * t511;
    (t5170, t5171, t5173, t5174, t5175, t5176, t5178, t5179)
}
