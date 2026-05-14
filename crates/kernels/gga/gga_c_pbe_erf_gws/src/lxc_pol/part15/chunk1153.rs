//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1153/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1153<F: Float>(t51383: F, t51401: F, t54293: F, t54294: F, t54295: F, t54297: F, t54299: F, t54302: F, t54303: F, t54305: F, t54307: F, t54310: F, t14101: F, t8842: F, t4028: F, t8856: F) -> (F, F, F) {
    let t54312 = -7.0 / 144.0 * t51383 - t54293 - t54294 + t54295 / 48.0 - t54297 / 24.0 + t54299 / 48.0 + t54302 + 5.0 / 192.0 * t54303 - 119.0 / 3456.0 * t54305 - t54307 / 48.0 - 35.0 / 576.0 * t51401 + t54310 / 192.0;
    let t54315 = t14101 * t8842;
    let t54317 = t4028 * t8856;
    (t54312, t54315, t54317)
}
