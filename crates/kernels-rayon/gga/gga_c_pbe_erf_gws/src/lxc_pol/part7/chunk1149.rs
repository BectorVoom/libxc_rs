//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1149/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1149(t6203: f64, t6347: f64, t4379: f64, t5: f64, t2147: f64, t337: f64, t2146: f64, t6253: f64, t6332: f64, t19562: f64, t346: f64, t2124: f64, t6800: f64) -> (f64, f64, f64, f64, f64) {
    let t20576 = t6203 * t6347;
    let t20578 = t5 * t4379;
    let t20580 = t2147 * t337 * t20578;
    let t20582 = t2146 * t20580 / 12.0_f64;
    let t20583 = t6253 * t6332;
    let t20584 = 7.0_f64 / 12.0_f64 * t20583;
    let t20585 = t19562 * t346;
    let t20588 = t6800 * t20585 * t2124 / 16.0_f64;
    (t20576, t20578, t20582, t20584, t20588)
}
