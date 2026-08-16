//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 851/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk851(t171: f64, t7337: f64, t145: f64, t169: f64, t242: f64, t2880: f64, t2887: f64, t2897: f64, t2932: f64, t2934: f64, t5745: f64, t5760: f64, t5770: f64, t5777: f64, t6037: f64, t6046: f64, t6052: f64, t7387: f64) -> (f64, f64) {
    let t7868 = t171 * t7337;
    let t7878 = t2880 - 0.42447554366239165_f64 * t5760 - t2887 + 0.15917832887339686_f64 * t6037 + 0.3183566577467937_f64 * t5777 + t2897 - 0.031835665774679375_f64 * t169 * t7868 * t242 - 0.09550699732403813_f64 * t6046 - 0.09550699732403813_f64 * t5770 - t2932 - t2934 + 0.9598512193592288_f64 * t5745 - 0.31995040645307626_f64 * t6052 + 0.05332506774217938_f64 * t145 * t7387;
    (t7868, t7878)
}
