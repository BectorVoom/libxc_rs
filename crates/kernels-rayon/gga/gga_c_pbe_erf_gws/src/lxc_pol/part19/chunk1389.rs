//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1389/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1389(t54321: f64, t55591: f64, t55593: f64, t57151: f64, t57154: f64, t57156: f64, t57158: f64, t57160: f64, t57162: f64, t57164: f64, t57166: f64, t57168: f64) -> f64 {
    let t58742 = t57151 / 96.0_f64 - t55591 - t54321 + t57154 / 24.0_f64 - t55593 - t57156 / 24.0_f64 - t57158 / 48.0_f64 + 7.0_f64 / 72.0_f64 * t57160 - t57162 / 48.0_f64 - t57164 / 48.0_f64 - t57166 / 48.0_f64 - t57168 / 384.0_f64;
    t58742
}
