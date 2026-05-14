//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1119/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1119<F: Float>(t29514: F, t7878: F, t22604: F, t15805: F, t25657: F, t58559: F, t6426: F, t373: F, t384: F, t4474: F, t58293: F, t11233: F, t3057: F, t58341: F, t29468: F, t383: F, t5537: F, t5546: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t115312 = t29514 * t7878;
    let t115316 = t29514 * t22604;
    let t115320 = t25657 * t15805;
    let t115324 = t6426 * t58559;
    let t115333 = t4474 * t373 * t384;
    let t115337 = t58293 * t384;
    let t115341 = t11233 * t3057;
    let t115349 = t58341 * t384;
    let t115353 = t29468 * t22604;
    let t115362 = t5537 * t5546 * t383;
    (t115312, t115316, t115320, t115324, t115333, t115337, t115341, t115349, t115353, t115362)
}
