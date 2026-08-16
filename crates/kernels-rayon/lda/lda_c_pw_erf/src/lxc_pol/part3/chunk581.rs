//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 581/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk581(t145: f64, t169: f64, t242: f64, t2880: f64, t2883: f64, t2887: f64, t2890: f64, t2893: f64, t2897: f64, t2898: f64, t2903: f64, t2906: f64, t2932: f64, t2934: f64, t2935: f64, t2937: f64, t3196: f64) -> f64 {
    let t3199 = t2880 - 0.42447554366239165_f64 * t2883 - t2887 + 0.15917832887339686_f64 * t2890 + 0.3183566577467937_f64 * t2893 + t2897 - 0.031835665774679375_f64 * t169 * t2898 * t242 - 0.09550699732403813_f64 * t2903 - 0.09550699732403813_f64 * t2906 - t2932 - t2934 + 0.9598512193592288_f64 * t2935 - 0.31995040645307626_f64 * t2937 + 0.05332506774217938_f64 * t145 * t3196;
    t3199
}
