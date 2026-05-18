//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1282/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1282<F: Float>(t4028: F, t9194: F, t4026: F, t828: F, t2137: F, t3108: F, t3287: F, t51255: F, t3142: F, t51382: F, t14007: F, t9421: F) -> (F, F, F, F, F) {
    let t54251 = t4028 * t9194;
    let t54253 = t4026 * t828;
    let t54255 = t3108 * t54253 * t2137;
    let t54257 = t51255 * t3287;
    let t54259 = t51382 * t3142;
    let t54261 = t14007 * t9421;
    (t54251, t54255, t54257, t54259, t54261)
}
