//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 906/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk906(t3671: f64, t513: f64, t3675: f64, t520: f64, t2919: f64, t985: f64, t3683: f64, t3644: f64, t481: f64, t2873: f64, t967: f64, t3637: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10134 = t3671 * t513;
    let t10144 = t3675 * t520;
    let t10147 = t985 * t2919;
    let t10151 = t3683 * t520;
    let t10154 = t3644 * t481;
    let t10158 = t967 * t2873;
    let t10162 = t3637 * t481;
    (t10134, t10144, t10147, t10151, t10154, t10158, t10162)
}
