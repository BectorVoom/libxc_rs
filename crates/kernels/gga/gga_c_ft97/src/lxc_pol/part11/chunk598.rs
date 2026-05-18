//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 598/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk598<F: Float>(t2: F, t7800: F, t7765: F, t1780: F, t3127: F, t7807: F, t1787: F, t7811: F, t7815: F, t1554: F, t369: F) -> (F, F, F, F, F, F, F) {
    let t8314 = t2 * t7800;
    let t8315 = t8314 * t7765;
    let t8316 = t1780 * t8315;
    let t8319 = t3127 * t7807;
    let t8322 = t1787 * t7811;
    let t8324 = t1787 * t7815;
    let t8326 = t1554 * t369;
    (t8314, t8315, t8316, t8319, t8322, t8324, t8326)
}
