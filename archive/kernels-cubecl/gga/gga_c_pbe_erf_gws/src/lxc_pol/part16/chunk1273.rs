//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1273/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1273<F: Float>(t14031: F, t9377: F, t3224: F, t51465: F, t1114: F, t51266: F, t6680: F, t2134: F, t8996: F, t14015: F, t9522: F, t1150: F, t51200: F) -> (F, F, F, F, F, F) {
    let t54115 = t14031 * t9377;
    let t54117 = t51465 * t3224;
    let t54119 = t1114 * t51266;
    let t54120 = t54119 * t6680;
    let t54122 = t2134 * t8996;
    let t54124 = t14015 * t9522;
    let t54126 = t51200 * t1150;
    (t54115, t54117, t54120, t54122, t54124, t54126)
}
