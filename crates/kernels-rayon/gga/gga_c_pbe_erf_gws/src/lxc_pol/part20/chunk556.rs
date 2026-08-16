//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 556/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk556(t513: f64, t981: f64, t520: f64, t985: f64, t133: f64, t2878: f64, t119: f64, t132: f64, t506: f64, t9: f64, t481: f64, t967: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2902 = t981 * t513;
    let t2905 = t985 * t520;
    let t2909 = t133 * t2878;
    let t2911 = t132 * t119;
    let t2912 = t9 * t506;
    let t2913 = t967 * t481;
    (t2902, t2905, t2909, t2911, t2912, t2913)
}
