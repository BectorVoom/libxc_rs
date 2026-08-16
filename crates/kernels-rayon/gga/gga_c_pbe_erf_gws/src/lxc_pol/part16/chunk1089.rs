//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1089/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1089(t13792: f64, t13984: f64, t2201: f64, t326: f64, t378: f64, t13952: f64, t886: f64) -> (f64, f64, f64, f64) {
    let t13985 = t13792 * t13984;
    let t13987 = t326 * t2201;
    let t13988 = t13987 * t378;
    let t14001 = t13952 * t886;
    (t13985, t13987, t13988, t14001)
}
