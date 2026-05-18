//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1342/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1342<F: Float>(t54319: F, t54322: F, t54329: F, t51408: F, t51412: F, t52696: F, t54315: F, t54317: F, t54324: F, t54326: F, t54333: F, t54335: F) -> F {
    let t55591 = F::new(7.0) / F::new(36.0) * t54319;
    let t55593 = F::new(7.0) / F::new(36.0) * t54322;
    let t55596 = F::new(7.0) / F::new(12.0) * t54329;
    let t55600 = t54315 / F::new(12.0) + t54317 / F::new(12.0) - t55591 - F::new(35.0) / F::new(108.0) * t51408 - t55593 - t54324 / F::new(48.0) - t54326 / F::new(96.0) - t55596 - F::new(35.0) / F::new(54.0) * t51412 - t52696 + t54333 / F::new(8.0) - t54335 / F::new(192.0);
    t55600
}
