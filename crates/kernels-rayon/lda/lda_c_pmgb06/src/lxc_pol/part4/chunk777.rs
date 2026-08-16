//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 777/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk777(t3227: f64, t3231: f64, t3233: f64, t3272: f64, t3274: f64, t1441: f64, t2002: f64, t3177: f64, t806: f64, t1420: f64, t2007: f64, t1980: f64, t431: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5157 = 4.0_f64 / 135.0_f64 * t3227;
    let t5158 = 2.0_f64 / 135.0_f64 * t3231;
    let t5159 = 2.0_f64 / 81.0_f64 * t3233;
    let t5160 = 2.0_f64 / 135.0_f64 * t3272;
    let t5161 = 2.0_f64 / 81.0_f64 * t3274;
    let t5163 = t2002 * t1441 / 27.0_f64;
    let t5165 = t3177 * t806 / 45.0_f64;
    let t5167 = 2.0_f64 / 45.0_f64 * t1420 * t2007;
    let t5168 = t431 * t1980;
    (t5157, t5158, t5159, t5160, t5161, t5163, t5165, t5167, t5168)
}
