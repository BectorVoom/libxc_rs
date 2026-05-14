//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 527/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk527<F: Float>(t364: F, t7773: F, t89: F, t1546: F, t1581: F, t1554: F, t375: F, t1560: F, t1558: F, t7765: F, t356: F, t1570: F, t363: F, t1580: F) -> (F, F, F, F, F, F, F, F) {
    let t7775 = t89 * t7773 * t364;
    let t7778 = t89 * t1546 * t1581;
    let t7780 = t375 * t1554;
    let t7782 = t89 * t7780 * t1560;
    let t7784 = t1558 * t7765;
    let t7786 = t89 * t356 * t7784;
    let t7788 = t1570 * t363;
    let t7789 = t7788 * t1580;
    (t7775, t7778, t7780, t7782, t7784, t7786, t7788, t7789)
}
