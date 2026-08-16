//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 475/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk475(t2200: f64, t333: f64, t338: f64, t348: f64, t837: f64, t855: f64, t863: f64) -> (f64, f64, f64) {
    let t2201 = t2200 * t333;
    let t2204 = 35.0_f64 / 432.0_f64 * t348 * t2201 * t338;
    let t2206 = t863 * t855 * t837;
    (t2201, t2204, t2206)
}
