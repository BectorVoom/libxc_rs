//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1151/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1151(t25354: f64, t1024: f64, t40790: f64, t42109: f64, t42131: f64, t3399: f64, t3456: f64, t16575: f64, t16577: f64, t16579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48313 = 64.0_f64 / 405.0_f64 * t25354;
    let t48315 = 16.0_f64 / 15.0_f64 * t40790 * t1024;
    let t48316 = 64.0_f64 / 45.0_f64 * t42109;
    let t48318 = 32.0_f64 / 15.0_f64 * t42131;
    let t48320 = 16.0_f64 / 5.0_f64 * t3399 * t3456;
    let t48321 = -t16575 - t16577 - t16579;
    (t48313, t48315, t48316, t48318, t48320, t48321)
}
