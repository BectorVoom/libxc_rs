//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 884/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk884<F: Float>(t2099: F, t2397: F, t2395: F, t6112: F, t6119: F, t6126: F, t6134: F, t6136: F, t6139: F, t6146: F, t6228: F, t6236: F, t6243: F, t6245: F) -> (F, F, F) {
    let t6491 = t2099 * t2397;
    let t6492 = t2395 * t6491;
    let t6494 = -t6112 - t6236 - t6228 + t6126 - t6243 - t6245 - t6119 + t6134 + t6136 + t6139 - t6146;
    (t6491, t6492, t6494)
}
