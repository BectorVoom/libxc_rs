//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 943/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk943(t145: f64, t169: f64, t242: f64, t5700: f64, t5723: f64, t5726: f64, t5730: f64, t5732: f64, t5735: f64, t8038: f64, t8363: f64, t8365: f64, t8371: f64, t8373: f64) -> f64 {
    let t8379 = -0.1066501354843587606e0_f64 * t5735 - 0.14149184788746388121e0_f64 * t8363 - 0.31835665774679373271e-1_f64 * t169 * t8365 * t242 - t8371 - 0.31835665774679373271e-1_f64 * t8373 + 0.533250677421793803e-1_f64 * t145 * t8038 - 0.31835665774679373271e-1_f64 * t5723 - 0.63671331549358746542e-1_f64 * t5726 - t5730 + t5700 - t5732;
    t8379
}
