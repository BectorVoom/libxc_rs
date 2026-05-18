//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1288/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1288<F: Float>(t51325: F, t54341: F, t14058: F, t3279: F, t4049: F, t9647: F, t4028: F, t9009: F, t9013: F, t1158: F, t51395: F, t3268: F) -> (F, F, F, F, F, F, F) {
    let t54342 = t54341 * t51325;
    let t54344 = t14058 * t3279;
    let t54346 = t4049 * t9647;
    let t54348 = t4028 * t9009;
    let t54350 = t4028 * t9013;
    let t54352 = t51395 * t1158;
    let t54354 = t14058 * t3268;
    (t54342, t54344, t54346, t54348, t54350, t54352, t54354)
}
