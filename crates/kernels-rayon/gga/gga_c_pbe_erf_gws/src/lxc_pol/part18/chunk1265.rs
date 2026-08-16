//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1265/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1265(t13893: f64, t4150: f64, t4002: f64, t8669: f64, t8743: f64, t13808: f64, t14596: f64, t53015: f64, t53334: f64, t53886: f64, t54094: f64, t54126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54724 = t13893 * t4150;
    let t54727 = 7.0_f64 / 144.0_f64 * t8669 * t4002;
    let t54729 = 7.0_f64 / 144.0_f64 * t8743 * t4002;
    let t54730 = t13808 * t14596;
    let t54731 = 7.0_f64 / 1152.0_f64 * t54730;
    let t54928 = 35.0_f64 / 216.0_f64 * t53015;
    let t55074 = 119.0_f64 / 6912.0_f64 * t53334;
    let t55408 = 119.0_f64 / 3456.0_f64 * t53886;
    let t55469 = 35.0_f64 / 216.0_f64 * t54094;
    let t55486 = 119.0_f64 / 1728.0_f64 * t54126;
    (t54724, t54727, t54729, t54731, t54928, t55074, t55408, t55469, t55486)
}
