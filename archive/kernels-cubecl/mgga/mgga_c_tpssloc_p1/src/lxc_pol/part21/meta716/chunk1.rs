//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2557/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2557<F: Float>(t10263: F, t4603: F, t10891: F, t13970: F, t10231: F, t13528: F, t973: F, t13532: F, t13537: F, t42972: F, t135: F, t14197: F) -> (F, F, F, F, F, F) {
    let t50098 = t10263 * t4603;
    let t50100 = t10891 * t13970;
    let t50110 = t973 * t10231 * t13528;
    let t50113 = t973 * t10231 * t13532;
    let t50116 = t973 * t42972 * t13537;
    let t50132 = t973 * t135 * t14197;
    (t50098, t50100, t50110, t50113, t50116, t50132)
}
