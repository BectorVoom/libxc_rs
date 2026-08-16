//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1139/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1139(t12325: f64, t13532: f64, t1619: f64, t2060: f64, t3103: f64, t9147: f64, t9149: f64, t9151: f64, t9179: f64, t9181: f64, t9184: f64, t9186: f64, t9215: f64, t9217: f64, t9679: f64, t9681: f64, t9683: f64, t9685: f64, t9687: f64, t9700: f64, t9702: f64) -> f64 {
    let t13554 = 0.10666666666666667_f64 * t13532 + 0.09597777777777777_f64 * t9147 + 0.07198333333333333_f64 * t9149 - 0.047988888888888886_f64 * t9179 - 0.03199259259259259_f64 * t9181 + 0.011997222222222222_f64 * t9184 + 0.013330246913580247_f64 * t9186 + 0.11197407407407407_f64 * t9215 - 0.047988888888888886_f64 * t9217 + 0.5038833333333333_f64 * t12325 + 0.044444444444444446_f64 * t9679 - 0.008888888888888889_f64 * t9681 - 0.007407407407407408_f64 * t9683 + 0.0044444444444444444_f64 * t9685 + 0.0019753086419753087_f64 * t9687 - 0.022222222222222223_f64 * t9700 + 0.05925925925925926_f64 * t9702 - 0.07198333333333333_f64 * t9151 - 0.013333333333333334_f64 * t2060 * t1619 * t3103;
    t13554
}
