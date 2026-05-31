//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1265/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1265<F: Float>(t13893: F, t4150: F, t4002: F, t8669: F, t8743: F, t13808: F, t14596: F, t53015: F, t53334: F, t53886: F, t54094: F, t54126: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54724 = t13893 * t4150;
    let t54727 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8669 * t4002;
    let t54729 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8743 * t4002;
    let t54730 = t13808 * t14596;
    let t54731 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t54730;
    let t54928 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t53015;
    let t55074 = F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t53334;
    let t55408 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t53886;
    let t55469 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t54094;
    let t55486 = F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t54126;
    (t54724, t54727, t54729, t54731, t54928, t55074, t55408, t55469, t55486)
}
