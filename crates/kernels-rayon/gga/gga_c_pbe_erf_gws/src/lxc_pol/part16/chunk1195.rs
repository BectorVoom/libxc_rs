//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1195/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1195(t14046: f64, t14096: f64, t2216: f64, t4033: f64, t14058: f64, t2327: f64, t14079: f64, t2285: f64, t1185: f64, t326: f64, t346: f64, t6045: f64) -> (f64, f64, f64, f64, f64) {
    let t51437 = t14046 * t14096;
    let t51439 = t4033 * t2216;
    let t51447 = t14058 * t2327;
    let t51452 = t14079 * t2285;
    let t51458 = t326 * t346 * t6045 * t1185;
    (t51437, t51439, t51447, t51452, t51458)
}
