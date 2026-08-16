//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1450/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1450(t11506: f64, t3014: f64, t41225: f64, t981: f64, t11610: f64, t3022: f64, t11396: f64, t3007: f64, t3033: f64, t11606: f64, t11571: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41481 = 0.6233709278045326953e3_f64 * t981 * t11506 * t41225 * t3014;
    let t41483 = 0.23392894490538584828e1_f64 * t3022 * t11610;
    let t41485 = 0.20779030926817756511e3_f64 * t3022 * t11396;
    let t41488 = 0.21053605041484726346e2_f64 * t981 * t3033 * t3007;
    let t41490 = 0.4155806185363551302e3_f64 * t3022 * t11606;
    let t41491 = t300 * t11571;
    (t41481, t41483, t41485, t41488, t41490, t41491)
}
