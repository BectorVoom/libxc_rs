//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1292/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1292<F: Float>(t16189: F, t16193: F, t16200: F, t16202: F, t16204: F, t16206: F, t16208: F, t16213: F, t16215: F, t16217: F, t16219: F, t16222: F, t16225: F, t16230: F, t16233: F, t16238: F, t16240: F) -> (F,) {
    let t19133 = t16189 + t16193 + t16200 - t16202 - t16204 - t16206 - t16208 - t16213 - t16215 - t16217 + t16219 - t16222 - t16225 + t16230 - t16233 + t16238 + t16240;
    (t19133,)
}
