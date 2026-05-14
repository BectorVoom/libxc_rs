//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1149/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1149<F: Float>(t2137: F, t3108: F, t54253: F, t3287: F, t51255: F, t3142: F, t51382: F, t14007: F, t9421: F, t51341: F, t51358: F, t54237: F, t54239: F, t54241: F, t54246: F, t54248: F, t54251: F) -> (F,) {
    let t54255 = t3108 * t54253 * t2137;
    let t54257 = t51255 * t3287;
    let t54258 = 7.0 / 144.0 * t54257;
    let t54259 = t51382 * t3142;
    let t54260 = 7.0 / 72.0 * t54259;
    let t54261 = t14007 * t9421;
    let t54263 = t54237 - t54239 - 7.0 / 72.0 * t51341 + t54241 / 48.0 + t54246 / 24.0 + t54248 / 192.0 - 7.0 / 288.0 * t51358 - t54251 / 16.0 - t54255 / 48.0 + t54258 - t54260 - t54261 / 768.0;
    (t54263,)
}
