//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1121/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1121<F: Float>(t54279: F, t9427: F, t3237: F, t51371: F, t3242: F, t14011: F, t9634: F, t3232: F, t4028: F, t9103: F, t14101: F, t8837: F, t9098: F, t14079: F, t3283: F, t4049: F, t9594: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t54280 = t54279 * t9427;
    let t54283 = t51371 * t3237;
    let t54285 = t51371 * t3242;
    let t54287 = t14011 * t9634;
    let t54289 = t51371 * t3232;
    let t54295 = t4028 * t9103;
    let t54297 = t14101 * t8837;
    let t54299 = t4028 * t9098;
    let t54301 = t14079 * t3283;
    let t54303 = t4049 * t9594;
    (t54280, t54283, t54285, t54287, t54289, t54295, t54297, t54299, t54301, t54303)
}
