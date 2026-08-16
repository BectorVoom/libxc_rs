//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1171/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1171(t19058: f64, t47391: f64, t3346: f64, t9778: f64, t1523: f64, t47409: f64, t12345: f64, t2477: f64, t47400: f64, t476: f64, t19071: f64, t47377: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48542 = t19058 * t47391;
    let t48544 = t9778 * t3346;
    let t48546 = t1523 * t47409;
    let t48548 = t2477 * t12345;
    let t48550 = t476 * t47400;
    let t48552 = t19071 * t47377;
    (t48542, t48544, t48546, t48548, t48550, t48552)
}
