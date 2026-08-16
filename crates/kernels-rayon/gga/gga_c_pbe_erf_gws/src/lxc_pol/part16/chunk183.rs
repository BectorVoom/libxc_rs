//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 183/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk183(t481: f64, t506: f64, t127: f64, t488: f64, t491: f64, t495: f64, t496: f64, t498: f64, t504: f64) -> f64 {
    let t507 = t506 * t481;
    let t510 = -t488 - t491 - t495 - t496 * t498 / 2.0_f64 - t504 - 0.146904e1_f64 * t127 * t507;
    t510
}
