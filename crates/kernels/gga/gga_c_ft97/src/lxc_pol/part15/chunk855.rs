//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 855/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk855<F: Float>(t1882: F, t21689: F, t21717: F, t761: F, t21439: F, t21405: F, t21417: F, t375: F, t89: F, t21399: F, t668: F, t21409: F, t2371: F, t21446: F, t9725: F, t21450: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t80477 = t1882 * t21689;
    let t80522 = t21717 * t761;
    let t80677 = t1882 * t21439;
    let t80679 = t1882 * t21405;
    let t80685 = t89 * t375 * t21417;
    let t80691 = t21399 * t668;
    let t80696 = t1882 * t21409;
    let t80748 = t2371 * t21399;
    let t80759 = t89 * t9725 * t21446;
    let t80770 = t1882 * t21450;
    (t80477, t80522, t80677, t80679, t80685, t80691, t80696, t80748, t80759, t80770)
}
