//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1087/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1087<F: Float>(t169: F, t242: F, t299: F, t7337: F, t462: F, t7387: F, t10878: F, t15237: F, t15245: F, t15247: F, t15251: F, t15253: F, t15257: F, t15259: F, t15270: F, t15275: F, t18934: F, t18942: F, t18945: F, t19008: F) -> F {
    let t20217 = t169 * t299 * t7337 * t242;
    let t20223 = t462 * t7387;
    let t20227 = F::cast_from(0.3183566577467937_f64) * t18934 - F::cast_from(0.09550699732403813_f64) * t18942 - F::cast_from(0.09550699732403813_f64) * t18945 + F::cast_from(0.053059442957798957_f64) * t20217 + F::cast_from(1.5564103267621028_f64) * t15237 + t15245 + F::cast_from(0.4775349866201906_f64) * t15247 - t15251 - F::cast_from(0.09550699732403813_f64) * t15253 - t15257 - F::cast_from(1.273426630987175_f64) * t15259 - F::cast_from(0.10665013548435875_f64) * t20223 + t10878 + F::cast_from(0.9598512193592288_f64) * t19008 - F::cast_from(3.839404877436915_f64) * t15270 + t15275;
    t20227
}
