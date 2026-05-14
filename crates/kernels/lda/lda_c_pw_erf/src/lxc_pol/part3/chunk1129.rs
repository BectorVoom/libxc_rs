//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1129/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1129<F: Float>(t462: F, t5718: F, t1891: F, t39: F, t10878: F, t10961: F, t11636: F, t14454: F, t145: F, t15237: F, t15241: F, t15245: F, t15247: F, t15251: F, t15253: F, t15257: F, t15259: F, t15266: F, t15270: F, t169: F, t171: F, t242: F) -> (F,) {
    let t15272 = t462 * t5718;
    let t15274 = t39 * t1891;
    let t15275 = 0.9598512193592288 * t15274;
    let t15276 = 0.9598512193592288 * t10961 + 0.5188034422540342 * t15237 + 0.15917832887339686 * t15241 + t15245 + 0.15917832887339686 * t15247 - t15251 - 0.031835665774679375 * t15253 - t15257 - 0.42447554366239165 * t15259 - 0.031835665774679375 * t169 * t171 * t11636 * t242 - 0.09550699732403813 * t15266 + 0.05332506774217938 * t145 * t14454 + t10878 - 1.279801625812305 * t15270 - 0.31995040645307626 * t15272 + t15275;
    (t15276,)
}
