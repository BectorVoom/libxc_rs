//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1324/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1324<F: Float>(t14397: F, t169: F, t242: F, t299: F, t4713: F, t5772: F, t632: F, t1143: F, t2220: F, t2224: F, t2929: F, t875: F) -> (F, F, F, F, F, F) {
    let t15237 = t169 * t14397 * t242;
    let t15241 = t169 * t299 * t4713 * t242;
    let t15244 = t169 * t5772 * t632;
    let t15245 = F::new(0.3183566577467937) * t15244;
    let t15247 = t169 * t2220 * t1143;
    let t15250 = t169 * t2224 * t1143;
    let t15251 = F::new(0.09550699732403813) * t15250;
    let t15253 = t169 * t875 * t2929;
    (t15237, t15241, t15245, t15247, t15251, t15253)
}
