//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 852/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk852(t553: f64, t6047: f64, t1996: f64, t5917: f64, t1472: f64, t2003: f64, t671: f64, t1750: f64, t1778: f64, t220: f64, t7776: f64, t211: f64) -> (f64, f64, f64, f64, f64) {
    let t16480 = 0.12408369628826103546e0_f64 * t6047 * t553;
    let t16481 = t1996 * t5917;
    let t16485 = 0.19878653761973934499e-1_f64 * t2003 * t1472 * t671;
    let t16486 = t1750 * t1778;
    let t16487 = 8.0_f64 / 45.0_f64 * t16486;
    let t16488 = t7776 * t220;
    let t16490 = 112.0_f64 / 1215.0_f64 * t211 * t16488;
    (t16480, t16481, t16485, t16487, t16490)
}
