//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 856/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk856(t1666: f64, t5304: f64, t196: f64, t5174: f64, t188: f64, t1804: f64, t185: f64, t186: f64, t1: f64, t3: f64, t4562: f64, t672: f64) -> (f64, f64, f64) {
    let t16529 = 16.0_f64 / 9.0_f64 * t5304 * t1666;
    let t16531 = 1.0_f64 / t5174 / t196;
    let t16532 = t188 * t16531;
    let t16533 = t1804 * t1804;
    let t16537 = 16.0_f64 / 5.0_f64 * t185 * t186 * t16532 * t16533;
    let t16540 = t4562 * t1 * t3 * t672;
    (t16529, t16537, t16540)
}
