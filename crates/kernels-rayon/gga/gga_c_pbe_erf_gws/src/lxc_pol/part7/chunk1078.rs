//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1078/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1078(t513: f64, t5842: f64, t1570: f64, t1576: f64, t510: f64, t5853: f64, t131: f64, t137: f64, t5852: f64, t1578: f64, t1590: f64, t133: f64, t19295: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19390 = t5842 * t513;
    let t19393 = t1570 * t1576;
    let t19398 = t510 * t5853;
    let t19407 = t131 / t5852 / t137;
    let t19408 = t1578 * t1578;
    let t19414 = t1590 * t1590;
    let t19420 = t133 * t19295;
    (t19390, t19393, t19398, t19407, t19408, t19414, t19420)
}
