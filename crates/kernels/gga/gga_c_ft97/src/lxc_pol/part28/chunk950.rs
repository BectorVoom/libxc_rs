//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 950/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk950<F: Float>(t136575: F, t7837: F, t1613: F, t58: F, t41: F, t8042: F, t1554: F, t373: F, t7205: F, t136559: F, t136565: F, t92470: F) -> (F, F, F, F, F) {
    let t136908 = t7837 * t136575;
    let t136918 = t1613 * t58;
    let t136920 = t8042 * t41 * t136918;
    let t136922 = t7205 * t1554 * t373;
    let t136926 = t8042 * t136559;
    let t136930 = t92470 * t136565;
    (t136908, t136920, t136922, t136926, t136930)
}
