//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 619/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk619(t2300: f64, t3189: f64, t904: f64, t3166: f64, t916: f64, t1123: f64, t2313: f64, t2255: f64, t2279: f64, t3258: f64, t3038: f64, t824: f64) -> (f64, f64, f64, f64, f64) {
    let t3279 = t2300 * t904 * t3189;
    let t3282 = t904 * t3166;
    let t3283 = t916 * t3282;
    let t3286 = t1123 * t2313;
    let t3287 = t2255 * t3286;
    let t3290 = t3258 * t2279;
    let t3291 = t2255 * t3290;
    let t3294 = t3038 * t824;
    (t3279, t3283, t3287, t3291, t3294)
}
