//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1209/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1209(t3222: f64, t37632: f64, t3327: f64, t810: f64, t332: f64, t4395: f64, t2157: f64, t938: f64, t2271: f64, t824: f64, t838: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46392 = t37632 * t3222;
    let t47184 = t3327 * t810;
    let t50887 = t4395 * t332;
    let t50912 = t2157 * t938;
    let t50935 = t2271 * t332;
    let t50942 = t824 * t838;
    let t50943 = t822 * t50942;
    (t46392, t47184, t50887, t50912, t50935, t50942, t50943)
}
