//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1249/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1249(t1114: f64, t51922: f64, t14001: f64, t3214: f64, t2370: f64, t3958: f64, t1144: f64, t13923: f64, t859: f64, t13911: f64, t26958: f64, t22336: f64, t4002: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53891 = t1114 * t51922;
    let t53896 = t14001 * t3214;
    let t53897 = 7.0_f64 / 72.0_f64 * t53896;
    let t53923 = t3958 * t2370;
    let t53939 = t859 * t1144 * t13923;
    let t53943 = 7.0_f64 / 72.0_f64 * t26958 * t13911;
    let t53948 = 7.0_f64 / 144.0_f64 * t22336 * t4002;
    (t53891, t53897, t53923, t53939, t53943, t53948)
}
