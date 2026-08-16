//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1330/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1330<F: Float>(t14031: F, t9382: F, t9552: F, t4028: F, t9116: F, t51459: F, t51461: F, t51466: F, t51473: F, t51479: F, t54391: F, t54394: F, t54398: F, t54402: F, t54404: F) -> F {
    let t54406 = t14031 * t9382;
    let t54408 = t14031 * t9552;
    let t54411 = t4028 * t9116;
    let t54413 = -t51459 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t51461 - t54391 / F::cast_from(4.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51466 - t54394 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51473 + t54398 - t54402 - t54404 / F::cast_from(96.0_f64) - t54406 / F::cast_from(384.0_f64) - t54408 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51479 - t54411 / F::cast_from(96.0_f64);
    t54413
}
