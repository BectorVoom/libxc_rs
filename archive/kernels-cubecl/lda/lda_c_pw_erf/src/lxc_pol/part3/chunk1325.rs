//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1325/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1325<F: Float>(t14403: F, t169: F, t242: F, t5466: F, t632: F, t5762: F, t343: F, t865: F, t462: F, t5718: F, t1891: F, t39: F) -> (F, F, F, F, F, F) {
    let t15256 = t169 * t14403 * t242;
    let t15257 = F::cast_from(0.42447554366239165_f64) * t15256;
    let t15259 = t169 * t5466 * t632;
    let t15266 = t169 * t5762 * t632;
    let t15270 = t343 * t865;
    let t15272 = t462 * t5718;
    let t15274 = t39 * t1891;
    (t15257, t15259, t15266, t15270, t15272, t15274)
}
