//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 825/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk825<F: Float>(t6122: F, t6230: F, t6233: F, t898: F, t6112: F, t6116: F, t6119: F, t6126: F, t6134: F, t6136: F, t6139: F, t6146: F, t6196: F, t6204: F, t6207: F, t6228: F) -> (F, F, F) {
    let t6234 = t6230 * t6122 * t6233;
    let t6236 = 0.10254018858216406658e4 * t898 * t6234;
    let t6237 = -t6112 + t6116 - t6119 + t6126 + t6134 + t6136 + t6139 - t6146 + t6196 + t6204 + t6207 - t6228 - t6236;
    (t6234, t6236, t6237)
}
