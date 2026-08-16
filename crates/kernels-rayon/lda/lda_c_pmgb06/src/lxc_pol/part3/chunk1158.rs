//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1158/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1158(t10335: f64, t10337: f64, t10339: f64, t10393: f64, t10343: f64, t10346: f64, t10348: f64, t10350: f64, t10353: f64, t10356: f64, t10358: f64, t10362: f64) -> (f64, f64, f64, f64, f64) {
    let t13822 = 8.0_f64 / 405.0_f64 * t10335;
    let t13823 = 4.0_f64 / 45.0_f64 * t10337;
    let t13824 = 4.0_f64 / 135.0_f64 * t10339;
    let t13829 = 4.0_f64 / 45.0_f64 * t10393;
    let t13830 = t13822 - t13823 + t13824 + t10343 + 0.36466666666666664_f64 * t10346 - 2.0_f64 / 9.0_f64 * t10348 - 2.0_f64 / 3.0_f64 * t10350 - 0.040518518518518516_f64 * t10353 - t10356 - t10358 + t10362 + t13829;
    (t13822, t13823, t13824, t13829, t13830)
}
