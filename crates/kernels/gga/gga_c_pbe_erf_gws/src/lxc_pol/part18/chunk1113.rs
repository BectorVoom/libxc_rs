//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1113/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1113<F: Float>(t4002: F, t8743: F, t13808: F, t14596: F, t53015: F, t53334: F, t53886: F, t54094: F, t54126: F, t54305: F, t54352: F, t54356: F, t54381: F, t54427: F, t54621: F, t54641: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t54729 = 7.0 / 144.0 * t8743 * t4002;
    let t54730 = t13808 * t14596;
    let t54731 = 7.0 / 1152.0 * t54730;
    let t54928 = 35.0 / 216.0 * t53015;
    let t55074 = 119.0 / 6912.0 * t53334;
    let t55408 = 119.0 / 3456.0 * t53886;
    let t55469 = 35.0 / 216.0 * t54094;
    let t55486 = 119.0 / 1728.0 * t54126;
    let t55582 = 119.0 / 1728.0 * t54305;
    let t55607 = 119.0 / 864.0 * t54352;
    let t55609 = 35.0 / 108.0 * t54356;
    let t55623 = 35.0 / 216.0 * t54381;
    let t55751 = 119.0 / 1728.0 * t54427;
    let t55892 = 35.0 / 216.0 * t54621;
    let t55947 = 35.0 / 216.0 * t54641;
    (t54729, t54731, t54928, t55074, t55408, t55469, t55486, t55582, t55607, t55609, t55623, t55751, t55892, t55947)
}
