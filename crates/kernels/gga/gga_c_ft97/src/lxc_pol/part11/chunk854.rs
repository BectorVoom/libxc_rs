//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 854/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk854<F: Float>(t1882: F, t9034: F, t9038: F, t9051: F, t9079: F, t1642: F, t1984: F, t1643: F, t1986: F, t446: F, t558: F, t7959: F, t9049: F, t2087: F, t2120: F, t91: F, t9252: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39685 = t1882 * t9034;
    let t39687 = t1882 * t9038;
    let t39689 = t1882 * t9051;
    let t39691 = t1882 * t9079;
    let t39693 = t1642 * t1984;
    let t39694 = t1643 * t1986;
    let t39696 = t446 * t39693 * t39694;
    let t39698 = t7959 * t558;
    let t39700 = t446 * t9049 * t39698;
    let t39704 = t91 * t9252 * t2087 * t2120;
    (t39685, t39687, t39689, t39691, t39694, t39696, t39698, t39700, t39704)
}
