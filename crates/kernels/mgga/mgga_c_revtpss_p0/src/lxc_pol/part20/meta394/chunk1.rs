//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1450/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1450<F: Float>(t11506: F, t3014: F, t41225: F, t981: F, t11610: F, t3022: F, t11396: F, t3007: F, t3033: F, t11606: F, t11571: F, t300: F) -> (F, F, F, F, F, F) {
    let t41481 = F::cast_from(0.6233709278045326953e3_f64) * t981 * t11506 * t41225 * t3014;
    let t41483 = F::cast_from(0.23392894490538584828e1_f64) * t3022 * t11610;
    let t41485 = F::cast_from(0.20779030926817756511e3_f64) * t3022 * t11396;
    let t41488 = F::cast_from(0.21053605041484726346e2_f64) * t981 * t3033 * t3007;
    let t41490 = F::cast_from(0.4155806185363551302e3_f64) * t3022 * t11606;
    let t41491 = t300 * t11571;
    (t41481, t41483, t41485, t41488, t41490, t41491)
}
