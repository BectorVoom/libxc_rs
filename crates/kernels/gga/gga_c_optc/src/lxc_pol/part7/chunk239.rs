//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 239/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk239<F: Float>(t661: F, t673: F, t678: F, t684: F, t686: F, t688: F, t695: F, t697: F, t703: F, t705: F) -> (F,) {
    let t708 = -0.86931614897887578546e-1 * t673 * t678 - t684 - 0.17386322979577515709e0 * t686 * t688 - 0.15114211337509259186e-1 * t695 * t697 - t703 - 0.30228422675018518372e-1 * t705 * t661;
    (t708,)
}
