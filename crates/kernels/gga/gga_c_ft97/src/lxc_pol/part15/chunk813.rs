//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 813/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk813<F: Float>(t38953: F, t4824: F, t4726: F, t8232: F, t4811: F, t4730: F, t1637: F, t4792: F, t89: F, t4815: F, t4735: F, t49266: F, t49337: F, t1526: F, t38308: F, t4641: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t63613 = t38953 * t4824;
    let t63746 = t8232 * t4726;
    let t63795 = t8232 * t4811;
    let t64001 = t8232 * t4730;
    let t64231 = t89 * t1637 * t4792;
    let t64255 = t8232 * t4815;
    let t64279 = t8232 * t4735;
    let t64491 = 56.0 / 81.0 * t49266;
    let t64516 = 56.0 / 243.0 * t49337;
    let t64663 = t1526 * t38308 * t4641;
    (t63613, t63746, t63795, t64001, t64231, t64255, t64279, t64491, t64516, t64663)
}
