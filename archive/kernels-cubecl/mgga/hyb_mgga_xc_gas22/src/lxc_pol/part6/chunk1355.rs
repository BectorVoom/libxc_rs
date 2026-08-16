//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1355/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1355<F: Float>(t29406: F, t29408: F, t29411: F, t29414: F, t29418: F, t29420: F, t29422: F, t29426: F, t29430: F, t29432: F, t29434: F, t29436: F, t29438: F) -> F {
    let t29510 = t29406 - t29408 + t29411 + t29414 + t29418 - t29420 + t29422 - t29426 + t29430 + t29432 + t29434 - t29436 - t29438;
    t29510
}
