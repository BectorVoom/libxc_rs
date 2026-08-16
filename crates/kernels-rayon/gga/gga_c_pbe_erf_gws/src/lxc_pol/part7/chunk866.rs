//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 866/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk866(t16641: f64, t1820: f64, t5539: f64, t7669: f64, t4897: f64, t5137: f64, t639: f64, t5342: f64, t586: f64, t1812: f64, t4913: f64, t5142: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16642 = 32.0_f64 / 27.0_f64 * t16641;
    let t16644 = t1820 * t7669 * t5539;
    let t16645 = 64.0_f64 / 27.0_f64 * t16644;
    let t16647 = t639 * t5137 * t4897;
    let t16648 = 32.0_f64 / 45.0_f64 * t16647;
    let t16649 = t5342 * t586;
    let t16651 = 32.0_f64 / 15.0_f64 * t16649 * t1812;
    let t16653 = 32.0_f64 / 15.0_f64 * t4913 * t5142;
    (t16642, t16645, t16648, t16649, t16651, t16653)
}
