//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 754/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk754(t3044: f64, t2089: f64, t489: f64, t161: f64, t2100: f64, t1600: f64, t842: f64, t1602: f64, t166: f64, t2018: f64, t486: f64, t2107: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5101 = 2.0_f64 / 45.0_f64 * t3044;
    let t5102 = t489 * t2089;
    let t5104 = 2.0_f64 / 45.0_f64 * t161 * t5102;
    let t5105 = t489 * t2100;
    let t5107 = 2.0_f64 / 45.0_f64 * t161 * t5105;
    let t5108 = t842 * t1600;
    let t5109 = t5108 * t1602;
    let t5110 = t166 * t5109;
    let t5112 = t161 * t5110 / 15.0_f64;
    let t5114 = 2.0_f64 / 45.0_f64 * t486 * t2018;
    let t5115 = t435 * t2107;
    (t5101, t5102, t5104, t5105, t5107, t5109, t5110, t5112, t5114, t5115)
}
