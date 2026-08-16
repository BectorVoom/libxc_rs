//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 389/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk389(t387: f64, t13: f64, t30: f64, t1275: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1289 = t387 * t387;
    let t1290 = 1.0_f64 / t1289;
    let t1291 = t13 * t1290;
    let t1292 = t30 * t30;
    let t1293 = 1.0_f64 / t1292;
    let t1294 = t1275 * t1293;
    let t1295 = t1291 * t1294;
    let t1296 = 0.16081824322151104822e2_f64 * t1295;
    (t1289, t1290, t1291, t1292, t1293, t1294, t1296)
}
