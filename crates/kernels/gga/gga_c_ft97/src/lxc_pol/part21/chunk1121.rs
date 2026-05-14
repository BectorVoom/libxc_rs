//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1121/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1121<F: Float>(t422: F, t4491: F, t22572: F, t29490: F, t5569: F, t29473: F, t7878: F, t22604: F, t11247: F, t3057: F, t384: F, t58348: F, t15776: F, t35: F, t373: F, t383: F, t925: F, t930: F) -> (F, F, F, F, F, F, F, F) {
    let t115370 = t422 * t4491;
    let t115379 = t5569 * t22572 * t29490;
    let t115381 = t29473 * t7878;
    let t115385 = t29473 * t22604;
    let t115389 = t11247 * t3057;
    let t115397 = t58348 * t384;
    let t115405 = t373 * t15776 * t35;
    let t115410 = t930 * t925 * t383;
    (t115370, t115379, t115381, t115385, t115389, t115397, t115405, t115410)
}
