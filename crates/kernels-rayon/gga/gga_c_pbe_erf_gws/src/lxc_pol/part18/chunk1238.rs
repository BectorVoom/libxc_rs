//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1238/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1238(t4146: f64, t51818: f64, t14592: f64, t50994: f64, t14749: f64, t9270: f64, t14643: f64, t840: f64, t14793: f64, t1144: f64, t13909: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53334 = t51818 * t4146;
    let t53353 = t50994 * t14592;
    let t53354 = 7.0_f64 / 288.0_f64 * t53353;
    let t53374 = 7.0_f64 / 72.0_f64 * t9270 * t14749;
    let t53405 = 7.0_f64 / 144.0_f64 * t840 * t14643;
    let t53407 = 7.0_f64 / 24.0_f64 * t9270 * t14793;
    let t53419 = t859 * t1144 * t13909;
    (t53334, t53354, t53374, t53405, t53407, t53419)
}
