//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1232/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1232(t13930: f64, t26958: f64, t14402: f64, t4386: f64, t892: f64, t50998: f64, t51066: f64, t9650: f64, t1105: f64, t353: f64, t4053: f64, t1193: f64, t2494: f64) -> (f64, f64, f64, f64, f64) {
    let t53028 = 7.0_f64 / 72.0_f64 * t26958 * t13930;
    let t53034 = t4386 * t892 * t14402;
    let t53038 = t50998 * t51066 * t9650;
    let t53042 = t4386 * t353 * t4053 * t1105;
    let t53047 = t4386 * t353 * t1193 * t2494;
    (t53028, t53034, t53038, t53042, t53047)
}
