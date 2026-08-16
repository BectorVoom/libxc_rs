//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 240/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk240(t661: f64, t673: f64, t678: f64, t684: f64, t686: f64, t688: f64, t695: f64, t697: f64, t703: f64, t705: f64) -> f64 {
    let t708 = -0.86931614897887578546e-1_f64 * t673 * t678 - t684 - 0.17386322979577515709e0_f64 * t686 * t688 - 0.15114211337509259186e-1_f64 * t695 * t697 - t703 - 0.30228422675018518372e-1_f64 * t705 * t661;
    t708
}
