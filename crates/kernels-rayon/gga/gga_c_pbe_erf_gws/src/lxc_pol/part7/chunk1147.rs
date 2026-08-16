//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1147/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1147(t6357: f64, t6416: f64, t6319: f64, t6605: f64, t2209: f64, t2365: f64, t6562: f64, t2146: f64, t6600: f64, t6258: f64, t6322: f64, t6203: f64, t6213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20546 = t6416 * t6357;
    let t20548 = t6319 * t6605;
    let t20549 = 7.0_f64 / 36.0_f64 * t20548;
    let t20550 = t2365 * t2209;
    let t20551 = t20550 * t6562;
    let t20552 = t2146 * t20551;
    let t20553 = 7.0_f64 / 4.0_f64 * t20552;
    let t20554 = t6416 * t6600;
    let t20557 = t6322 * t6258 / 8.0_f64;
    let t20558 = t6203 * t6213;
    (t20546, t20549, t20553, t20554, t20557, t20558)
}
