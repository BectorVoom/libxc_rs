//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1167/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1167<F: Float>(t54319: F, t54322: F, t54329: F, t51408: F, t51412: F, t52696: F, t54315: F, t54317: F, t54324: F, t54326: F, t54333: F, t54335: F, t54344: F, t54352: F, t54354: F, t54356: F) -> (F, F, F, F, F) {
    let t55591 = 7.0 / 36.0 * t54319;
    let t55593 = 7.0 / 36.0 * t54322;
    let t55596 = 7.0 / 12.0 * t54329;
    let t55600 = t54315 / 12.0 + t54317 / 12.0 - t55591 - 35.0 / 108.0 * t51408 - t55593 - t54324 / 48.0 - t54326 / 96.0 - t55596 - 35.0 / 54.0 * t51412 - t52696 + t54333 / 8.0 - t54335 / 192.0;
    let t55603 = 35.0 / 144.0 * t54344;
    let t55607 = 119.0 / 864.0 * t54352;
    let t55608 = 7.0 / 144.0 * t54354;
    let t55609 = 35.0 / 108.0 * t54356;
    (t55600, t55603, t55607, t55608, t55609)
}
