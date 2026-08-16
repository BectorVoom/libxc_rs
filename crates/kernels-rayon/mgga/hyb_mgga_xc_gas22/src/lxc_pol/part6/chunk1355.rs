//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1355/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1355(t29406: f64, t29408: f64, t29411: f64, t29414: f64, t29418: f64, t29420: f64, t29422: f64, t29426: f64, t29430: f64, t29432: f64, t29434: f64, t29436: f64, t29438: f64) -> f64 {
    let t29510 = t29406 - t29408 + t29411 + t29414 + t29418 - t29420 + t29422 - t29426 + t29430 + t29432 + t29434 - t29436 - t29438;
    t29510
}
