//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1322/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1322<F: Float>(t14101: F, t8842: F, t4028: F, t8856: F, t14046: F, t3184: F, t51408: F, t3148: F, t3123: F, t51309: F, t14015: F, t9467: F) -> (F, F, F, F, F, F, F) {
    let t54315 = t14101 * t8842;
    let t54317 = t4028 * t8856;
    let t54319 = t14046 * t3184;
    let t54320 = F::new(7.0) / F::new(72.0) * t54319;
    let t54321 = F::new(35.0) / F::new(216.0) * t51408;
    let t54322 = t14046 * t3148;
    let t54323 = F::new(7.0) / F::new(72.0) * t54322;
    let t54324 = t3123 * t51309;
    let t54326 = t14015 * t9467;
    (t54315, t54317, t54320, t54321, t54323, t54324, t54326)
}
