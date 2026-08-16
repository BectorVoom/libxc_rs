//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 750/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk750(t2085: f64, t4913: f64, t1832: f64, t4641: f64, t497: f64, t524: f64, t165: f64, t138: f64, t2897: f64, t146: f64, t2060: f64, t2901: f64, t2903: f64, t2905: f64, t2907: f64, t3336: f64, t3350: f64, t3352: f64, t3354: f64, t3365: f64, t3368: f64, t3380: f64, t4906: f64, t4909: f64, t4911: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4914 = t4913 * t2085;
    let t4916 = t4641 * t1832;
    let t4918 = t524 * t497;
    let t4922 = t165 * t497;
    let t4924 = t138 * t2897 * t4922;
    let t4934 = 0.008888888888888889_f64 * t2060 * t4906 - 0.007407407407407408_f64 * t4909 - 0.015996296296296297_f64 * t4911 - 0.057777777777777775_f64 * t4914 - 0.2639388888888889_f64 * t4916 + 0.013333333333333334_f64 * t146 * t3365 * t4918 + 0.07198333333333333_f64 * t4924 - 0.008888888888888889_f64 * t3336 - 0.014814814814814815_f64 * t3350 + 0.0044444444444444444_f64 * t3352 + 0.0014814814814814814_f64 * t3354 - 0.023994444444444443_f64 * t2905 - 0.03199259259259259_f64 * t2901 + 0.011997222222222222_f64 * t2907 + 0.007998148148148148_f64 * t2903 - t3368 - t3380;
    (t4914, t4916, t4918, t4922, t4924, t4934)
}
