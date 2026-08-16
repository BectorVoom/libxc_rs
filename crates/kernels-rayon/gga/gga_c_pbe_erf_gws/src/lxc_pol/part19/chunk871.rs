//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 871/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk871(t3028: f64, t817: f64, t1109: f64, t2106: f64, t1140: f64, t6480: f64, t1125: f64, t6616: f64, t2145: f64, t3039: f64, t19: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9147 = t3028 * t817;
    let t9150 = t1109 * t2106;
    let t9176 = t6480 * t1140;
    let t9182 = t1125 * t6616;
    let t9188 = t3039 * t2145;
    let t9239 = t931 * t19;
    (t9147, t9150, t9176, t9182, t9188, t9239)
}
