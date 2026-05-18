//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1323/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1323<F: Float>(t14023: F, t14548: F, t863: F, t51412: F, t14547: F, t28029: F, t6523: F, t14031: F, t9556: F, t51415: F, t54315: F, t54317: F, t54320: F, t54321: F, t54323: F, t54324: F, t54326: F) -> F {
    let t54329 = t863 * t14023 * t14548;
    let t54330 = F::new(7.0) / F::new(24.0) * t54329;
    let t54331 = F::new(35.0) / F::new(108.0) * t51412;
    let t54333 = t14547 * t6523 * t28029;
    let t54335 = t14031 * t9556;
    let t54337 = t54315 / F::new(24.0) + t54317 / F::new(24.0) - t54320 - t54321 - t54323 - t54324 / F::new(96.0) - t54326 / F::new(192.0) - t54330 - t54331 - t51415 + t54333 / F::new(16.0) - t54335 / F::new(384.0);
    t54337
}
