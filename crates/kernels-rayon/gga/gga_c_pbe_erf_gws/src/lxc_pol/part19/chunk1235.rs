//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1235/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1235(t14425: f64, t51563: f64, t4138: f64, t50948: f64, t1114: f64, t51922: f64, t14001: f64, t3214: f64, t51819: f64, t2370: f64, t3958: f64, t14784: f64, t50994: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53873 = t51563 * t14425;
    let t53886 = t50948 * t4138;
    let t53891 = t1114 * t51922;
    let t53896 = t14001 * t3214;
    let t53915 = 119.0_f64 / 6912.0_f64 * t51819;
    let t53923 = t3958 * t2370;
    let t53952 = t50994 * t14784;
    (t53873, t53886, t53891, t53896, t53915, t53923, t53952)
}
