//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1166/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1166<F: Float>(t1278: F, t1313: F, t2437: F, t519: F, t1446: F, t6332: F, t1326: F, t15853: F, t9909: F, t12747: F, t9923: F, t9925: F, t17140: F, t17143: F, t17146: F, t17149: F, t17151: F, t17154: F, t17157: F, t17159: F, t17163: F) -> (F, F, F, F, F, F, F, F) {
    let t17167 = 4.0 / 45.0 * t519 * t1313 * t2437 * t1278;
    let t17169 = 16.0 / 45.0 * t1446 * t6332;
    let t17172 = 16.0 / 45.0 * t519 * t1326 * t15853;
    let t17173 = 16.0 / 405.0 * t9909;
    let t17174 = 8.0 / 45.0 * t12747;
    let t17175 = 4.0 / 135.0 * t9923;
    let t17176 = 16.0 / 135.0 * t9925;
    let t17177 = t17140 + t17143 - t17146 - t17149 + t17151 + t17154 + t17157 - t17159 - t17163 - t17167 - t17169 - t17172 - t17173 - t17174 + t17175 - t17176;
    (t17167, t17169, t17172, t17173, t17174, t17175, t17176, t17177)
}
