//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1087/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1087(t169: f64, t242: f64, t299: f64, t7337: f64, t462: f64, t7387: f64, t10878: f64, t15237: f64, t15245: f64, t15247: f64, t15251: f64, t15253: f64, t15257: f64, t15259: f64, t15270: f64, t15275: f64, t18934: f64, t18942: f64, t18945: f64, t19008: f64) -> f64 {
    let t20217 = t169 * t299 * t7337 * t242;
    let t20223 = t462 * t7387;
    let t20227 = 0.3183566577467937_f64 * t18934 - 0.09550699732403813_f64 * t18942 - 0.09550699732403813_f64 * t18945 + 0.053059442957798957_f64 * t20217 + 1.5564103267621028_f64 * t15237 + t15245 + 0.4775349866201906_f64 * t15247 - t15251 - 0.09550699732403813_f64 * t15253 - t15257 - 1.273426630987175_f64 * t15259 - 0.10665013548435875_f64 * t20223 + t10878 + 0.9598512193592288_f64 * t19008 - 3.839404877436915_f64 * t15270 + t15275;
    t20227
}
