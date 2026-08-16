//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 996/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk996<F: Float>(t2187: F, t8232: F, t1882: F, t9260: F, t3281: F, t576: F, t611: F, t558: F, t7765: F, t8392: F, t9345: F, t1559: F, t1986: F) -> (F, F, F, F, F, F, F) {
    let t40685 = t8232 * t2187;
    let t40690 = t1882 * t9260;
    let t40696 = t3281 * t576;
    let t40698 = t3281 * t611;
    let t40700 = t7765 * t558;
    let t40720 = t8392 * t9345;
    let t40722 = t1559 * t1986;
    (t40685, t40690, t40696, t40698, t40700, t40720, t40722)
}
