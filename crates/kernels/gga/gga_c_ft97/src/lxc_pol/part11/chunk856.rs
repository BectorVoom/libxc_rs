//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 856/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk856<F: Float>(t2205: F, t37320: F, t446: F, t1651: F, t1986: F, t9073: F, t1882: F, t9046: F, t558: F, t7966: F, t1969: F, t1971: F, t8232: F, t525: F, t7954: F, t7955: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39711 = t446 * t2205 * t37320;
    let t39713 = t1651 * t1986;
    let t39715 = t446 * t9073 * t39713;
    let t39717 = t1882 * t9046;
    let t39719 = t7966 * t558;
    let t39721 = t446 * t1969 * t39719;
    let t39723 = t8232 * t1971;
    let t39725 = t7954 * t525;
    let t39726 = t7955 * t558;
    (t39711, t39713, t39715, t39717, t39719, t39721, t39723, t39725, t39726)
}
