//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 925/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk925<F: Float>(t12196: F, t1472: F, t4838: F, t1308: F, t3655: F, t571: F, t816: F, t10474: F, t2027: F, t3794: F, t4826: F, t12176: F, t12178: F, t12181: F, t12184: F, t12186: F, t12188: F, t12190: F, t12194: F) -> (F, F, F, F, F, F) {
    let t12197 = 8.0 / 135.0 * t12196;
    let t12199 = 4.0 / 15.0 * t1472 * t4838;
    let t12203 = 4.0 / 45.0 * t571 * t1308 * t816 * t3655;
    let t12205 = 8.0 / 15.0 * t10474 * t2027;
    let t12207 = 8.0 / 15.0 * t3794 * t4826;
    let t12208 = -t12176 - t12178 - t12181 - t12184 - t12186 - t12188 - t12190 - t12194 + t12197 - t12199 - t12203 + t12205 + t12207;
    (t12197, t12199, t12203, t12205, t12207, t12208)
}
