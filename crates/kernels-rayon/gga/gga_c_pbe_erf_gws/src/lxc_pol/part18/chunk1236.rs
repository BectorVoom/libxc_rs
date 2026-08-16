//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1236/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1236(t1114: f64, t50942: f64, t13984: f64, t3308: f64, t859: f64, t3973: f64, t3991: f64, t15641: f64, t3098: f64, t4386: f64, t3316: f64, t1192: f64, t20173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t53229 = t1114 * t50942;
    let t53230 = t53229 * t13984;
    let t53231 = 7.0_f64 / 144.0_f64 * t53230;
    let t53233 = t859 * t3308;
    let t53236 = t3973 * t3991;
    let t53240 = t3973 * t15641;
    let t53245 = t4386 * t3098;
    let t53250 = t859 * t3316;
    let t53253 = t20173 * t1192;
    (t53229, t53231, t53233, t53236, t53240, t53245, t53250, t53253)
}
