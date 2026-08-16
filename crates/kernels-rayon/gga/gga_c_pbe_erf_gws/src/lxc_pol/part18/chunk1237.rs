//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1237/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1237(t14125: f64, t3111: f64, t833: f64, t850: f64, t14617: f64, t50943: f64, t345: f64, t6126: f64, t14669: f64, t9270: f64, t14448: f64, t4414: f64) -> (f64, f64, f64, f64, f64) {
    let t53260 = t850 * t3111 * t14125 * t833;
    let t53261 = 7.0_f64 / 144.0_f64 * t53260;
    let t53272 = t50943 * t14617;
    let t53273 = 7.0_f64 / 144.0_f64 * t53272;
    let t53283 = t345 * t6126;
    let t53302 = 7.0_f64 / 72.0_f64 * t9270 * t14669;
    let t53308 = 7.0_f64 / 72.0_f64 * t4414 * t14448;
    (t53261, t53273, t53283, t53302, t53308)
}
