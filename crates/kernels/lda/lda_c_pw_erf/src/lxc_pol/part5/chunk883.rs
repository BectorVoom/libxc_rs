//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 883/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk883<F: Float>(t1217: F, t2281: F, t3704: F, t858: F, t14397: F, t169: F, t242: F, t5772: F, t632: F, t1143: F, t2220: F, t2224: F, t2929: F, t875: F, t14403: F, t5466: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15151 = t2281 * t1217;
    let t15152 = 2.0 / 45.0 * t15151;
    let t15153 = t858 * t3704;
    let t15237 = t169 * t14397 * t242;
    let t15244 = t169 * t5772 * t632;
    let t15245 = 0.3183566577467937 * t15244;
    let t15247 = t169 * t2220 * t1143;
    let t15250 = t169 * t2224 * t1143;
    let t15251 = 0.09550699732403813 * t15250;
    let t15253 = t169 * t875 * t2929;
    let t15256 = t169 * t14403 * t242;
    let t15257 = 0.42447554366239165 * t15256;
    let t15259 = t169 * t5466 * t632;
    (t15152, t15153, t15237, t15245, t15247, t15251, t15253, t15257, t15259)
}
