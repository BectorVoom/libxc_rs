//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1385/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1385(t55547: f64, t55548: f64, t57060: f64, t57062: f64, t57064: f64, t57066: f64, t57068: f64, t57070: f64, t57073: f64, t57075: f64, t57077: f64, t57079: f64) -> f64 {
    let t58697 = -t57060 / 12.0_f64 - t57062 / 96.0_f64 - t57064 / 24.0_f64 + t57066 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t57068 + t57070 / 96.0_f64 + t55547 - t57073 / 48.0_f64 - t57075 / 96.0_f64 - t57077 / 12.0_f64 - t55548 - t57079 / 48.0_f64;
    t58697
}
