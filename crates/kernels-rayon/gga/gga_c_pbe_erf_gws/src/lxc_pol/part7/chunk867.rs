//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 867/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk867(t4903: f64, t4913: f64, t5156: f64, t16630: f64, t16633: f64, t16636: f64, t16639: f64, t16642: f64, t16645: f64, t16648: f64, t16651: f64, t16653: f64) -> (f64, f64, f64) {
    let t16655 = 32.0_f64 / 15.0_f64 * t4913 * t4903;
    let t16657 = 32.0_f64 / 9.0_f64 * t4913 * t5156;
    let t16658 = t16630 - t16633 - t16636 - t16639 + t16642 + t16645 - t16648 + t16651 + t16653 + t16655 + t16657;
    (t16655, t16657, t16658)
}
