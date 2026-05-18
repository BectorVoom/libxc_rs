//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 993/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk993<F: Float>(t15250: F, t169: F, t2929: F, t875: F, t14403: F, t242: F, t5466: F, t632: F, t343: F, t865: F, t1891: F, t39: F) -> (F, F, F, F, F, F) {
    let t15251 = F::new(0.09550699732403813) * t15250;
    let t15253 = t169 * t875 * t2929;
    let t15256 = t169 * t14403 * t242;
    let t15257 = F::new(0.42447554366239165) * t15256;
    let t15259 = t169 * t5466 * t632;
    let t15270 = t343 * t865;
    let t15274 = t39 * t1891;
    (t15251, t15253, t15257, t15259, t15270, t15274)
}
