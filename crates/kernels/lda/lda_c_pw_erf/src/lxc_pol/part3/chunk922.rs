//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 922/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk922<F: Float>(t4675: F, t954: F, t4868: F, t571: F, t219: F, t4048: F, t473: F, t34: F, t3589: F, t951: F, t1472: F, t4873: F, t1278: F, t1976: F, t4848: F, t519: F) -> (F, F, F, F, F, F) {
    let t12153 = t4675 * t954;
    let t12156 = 8.0 / 9.0 * t571 * t4868 * t12153;
    let t12158 = t473 * t4048 * t219;
    let t12160 = t3589 * t34 * t951;
    let t12163 = 64.0 / 27.0 * t571 * t12158 * t12160;
    let t12165 = 4.0 / 15.0 * t1472 * t4873;
    let t12169 = 8.0 / 15.0 * t519 * t4848 * t1976 * t1278;
    (t12153, t12156, t12160, t12163, t12165, t12169)
}
