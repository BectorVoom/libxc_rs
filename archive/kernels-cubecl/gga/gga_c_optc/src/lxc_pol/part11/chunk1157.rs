//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1157/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1157<F: Float>(t10856: F, t17160: F, t3907: F, t11380: F, t17114: F, t3884: F, t2674: F, t51745: F, t8134: F, t50760: F, t953: F, t17056: F, t2367: F, t930: F) -> (F, F, F, F, F) {
    let t51780 = t3907 * t10856 * t17160;
    let t51785 = t3884 * t11380 * t17114;
    let t51788 = t8134 * t51745 * t2674;
    let t51790 = t953 * t50760;
    let t51819 = t930 * t2367 * t17056;
    (t51780, t51785, t51788, t51790, t51819)
}
