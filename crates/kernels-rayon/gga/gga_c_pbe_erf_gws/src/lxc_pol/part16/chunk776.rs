//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 776/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk776(t2031: f64, t5628: f64, t168: f64, t5589: f64, t286: f64, t2030: f64, t522: f64, t475: f64, t137: f64, t142: f64, t481: f64, t510: f64) -> (f64, f64, f64, f64, f64) {
    let t5629 = t2031 * t5628;
    let t5631 = t168 * t5589;
    let t5633 = 0.19513566535229733338e0_f64 * t5631 * t286;
    let t5649 = t522 * t2030;
    let t5650 = t475 * t5649;
    let t5651 = t137 * t142;
    let t5652 = t510 * t481;
    (t5629, t5633, t5650, t5651, t5652)
}
