//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 891/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk891<F: Float>(t3281: F, t571: F, t2218: F, t8232: F, t2207: F, t1882: F, t9416: F, t2202: F, t2187: F, t9260: F, t576: F, t611: F, t558: F, t7765: F, t8392: F, t9345: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t40671 = t3281 * t571;
    let t40673 = t8232 * t2218;
    let t40675 = t8232 * t2207;
    let t40677 = t1882 * t9416;
    let t40679 = t8232 * t2202;
    let t40685 = t8232 * t2187;
    let t40690 = t1882 * t9260;
    let t40696 = t3281 * t576;
    let t40698 = t3281 * t611;
    let t40700 = t7765 * t558;
    let t40720 = t8392 * t9345;
    (t40671, t40673, t40675, t40677, t40679, t40685, t40690, t40696, t40698, t40700, t40720)
}
