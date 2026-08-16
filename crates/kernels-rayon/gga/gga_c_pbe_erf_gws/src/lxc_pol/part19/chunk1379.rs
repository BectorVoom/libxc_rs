//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1379/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1379(t54038: f64, t54094: f64, t55452: f64, t55460: f64, t55467: f64, t56910: f64, t56912: f64, t56914: f64, t56917: f64, t56920: f64, t56922: f64, t56924: f64, t56926: f64) -> f64 {
    let t58619 = t54038 + t56910 / 24.0_f64 - t55452 + t56912 / 96.0_f64 + t56914 / 12.0_f64 + t55460 + t56917 / 24.0_f64 - t56920 / 48.0_f64 + 7.0_f64 / 576.0_f64 * t56922 + t55467 + 35.0_f64 / 108.0_f64 * t54094 + t56924 / 96.0_f64 - t56926 / 384.0_f64;
    t58619
}
