//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1106/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1106<F: Float>(t16127: F, t4804: F, t6292: F, t6230: F, t3859: F, t519: F, t6492: F, t1326: F, t15881: F, t12695: F, t6442: F, t16093: F, t16095: F, t16099: F, t16103: F, t16108: F, t16110: F, t16114: F, t16116: F, t16118: F, t16120: F, t16125: F) -> (F, F, F, F, F, F, F) {
    let t16128 = 32.0 / 27.0 * t16127;
    let t16129 = t4804 * t6292;
    let t16130 = 64.0 / 135.0 * t16129;
    let t16132 = 32.0 / 45.0 * t4804 * t6230;
    let t16134 = t519 * t3859 * t6492;
    let t16135 = 32.0 / 45.0 * t16134;
    let t16138 = 32.0 / 15.0 * t519 * t1326 * t15881;
    let t16140 = t519 * t12695 * t6442;
    let t16141 = 32.0 / 27.0 * t16140;
    let t16142 = -t16093 + t16095 - t16099 + t16103 + t16108 + t16110 + t16114 + t16116 + t16118 - t16120 - t16125 - t16128 + t16130 + t16132 + t16135 - t16138 + t16141;
    (t16128, t16130, t16132, t16135, t16138, t16141, t16142)
}
