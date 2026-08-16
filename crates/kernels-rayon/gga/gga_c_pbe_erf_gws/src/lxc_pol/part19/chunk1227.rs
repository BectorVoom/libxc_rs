//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1227/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1227(t3308: f64, t859: f64, t3973: f64, t3991: f64, t15641: f64, t3098: f64, t4386: f64, t3316: f64, t14125: f64, t3111: f64, t833: f64, t850: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53233 = t859 * t3308;
    let t53236 = t3973 * t3991;
    let t53240 = t3973 * t15641;
    let t53245 = t4386 * t3098;
    let t53250 = t859 * t3316;
    let t53260 = t850 * t3111 * t14125 * t833;
    (t53233, t53236, t53240, t53245, t53250, t53260)
}
