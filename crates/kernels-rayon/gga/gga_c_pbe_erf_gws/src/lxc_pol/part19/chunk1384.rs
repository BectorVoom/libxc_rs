//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1384/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1384(t55524: f64, t57028: f64, t57031: f64, t57036: f64, t57038: f64, t57040: f64, t57042: f64, t57044: f64, t57046: f64, t57048: f64, t57050: f64, t57052: f64, t57054: f64) -> f64 {
    let t58683 = t57028 / 24.0_f64 - t57031 / 24.0_f64 + t57036 / 24.0_f64 - t57038 / 24.0_f64 - t57040 / 24.0_f64 - t57042 / 192.0_f64 - t55524 + t57044 / 4.0_f64 - t57046 / 24.0_f64 - t57048 / 48.0_f64 + t57050 / 96.0_f64 + t57052 / 64.0_f64 - t57054 / 12.0_f64;
    t58683
}
