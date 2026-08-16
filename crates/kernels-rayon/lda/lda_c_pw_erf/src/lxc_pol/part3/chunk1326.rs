//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1326/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1326(t15274: f64, t10878: f64, t10961: f64, t11636: f64, t14454: f64, t145: f64, t15237: f64, t15241: f64, t15245: f64, t15247: f64, t15251: f64, t15253: f64, t15257: f64, t15259: f64, t15266: f64, t15270: f64, t15272: f64, t169: f64, t171: f64, t242: f64) -> f64 {
    let t15275 = 0.9598512193592288_f64 * t15274;
    let t15276 = 0.9598512193592288_f64 * t10961 + 0.5188034422540342_f64 * t15237 + 0.15917832887339686_f64 * t15241 + t15245 + 0.15917832887339686_f64 * t15247 - t15251 - 0.031835665774679375_f64 * t15253 - t15257 - 0.42447554366239165_f64 * t15259 - 0.031835665774679375_f64 * t169 * t171 * t11636 * t242 - 0.09550699732403813_f64 * t15266 + 0.05332506774217938_f64 * t145 * t14454 + t10878 - 1.279801625812305_f64 * t15270 - 0.31995040645307626_f64 * t15272 + t15275;
    t15276
}
