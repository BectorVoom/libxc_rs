//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1155/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1155<F: Float>(t51415: F, t54315: F, t54317: F, t54320: F, t54321: F, t54323: F, t54324: F, t54326: F, t54330: F, t54331: F, t54333: F, t54335: F, t14011: F, t9344: F, t850: F, t852: F, t9441: F) -> (F, F, F) {
    let t54337 = t54315 / 24.0 + t54317 / 24.0 - t54320 - t54321 - t54323 - t54324 / 96.0 - t54326 / 192.0 - t54330 - t54331 - t51415 + t54333 / 16.0 - t54335 / 384.0;
    let t54338 = t14011 * t9344;
    let t54341 = t850 * t9441 * t852;
    (t54337, t54338, t54341)
}
