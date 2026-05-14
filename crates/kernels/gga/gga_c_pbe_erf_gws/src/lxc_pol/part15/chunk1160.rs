//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1160/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1160<F: Float>(t14031: F, t9552: F, t4028: F, t9116: F, t51459: F, t51461: F, t51466: F, t51473: F, t51479: F, t54391: F, t54394: F, t54398: F, t54402: F, t54404: F, t54406: F, t4142: F, t51529: F) -> (F, F) {
    let t54408 = t14031 * t9552;
    let t54411 = t4028 * t9116;
    let t54413 = -t51459 - 7.0 / 48.0 * t51461 - t54391 / 4.0 + 7.0 / 288.0 * t51466 - t54394 / 16.0 + 7.0 / 288.0 * t51473 + t54398 - t54402 - t54404 / 96.0 - t54406 / 384.0 - t54408 / 384.0 + 7.0 / 1152.0 * t51479 - t54411 / 96.0;
    let t54427 = t51529 * t4142;
    (t54413, t54427)
}
