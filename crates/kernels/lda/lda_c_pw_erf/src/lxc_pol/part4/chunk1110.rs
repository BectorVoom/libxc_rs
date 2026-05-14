//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1110/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1110<F: Float>(t1278: F, t1325: F, t5289: F, t6431: F, t2031: F, t5327: F, t2171: F, t5244: F, t4850: F, t1987: F, t504: F, t6566: F, t1313: F, t348: F, t519: F, t2146: F, t4873: F) -> (F, F, F, F, F, F, F) {
    let t16200 = 16.0 / 15.0 * t1325 * t5289 * t6431 * t1278;
    let t16202 = 16.0 / 45.0 * t5327 * t2031;
    let t16204 = 8.0 / 45.0 * t2171 * t5244;
    let t16206 = 32.0 / 45.0 * t2171 * t4850;
    let t16208 = 32.0 / 45.0 * t5327 * t1987;
    let t16209 = t6566 * t504;
    let t16213 = 8.0 / 45.0 * t519 * t1313 * t16209 * t348;
    let t16215 = 8.0 / 45.0 * t2146 * t4873;
    (t16200, t16202, t16204, t16206, t16208, t16213, t16215)
}
