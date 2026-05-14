//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 951/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk951<F: Float>(t237: F, t7266: F, t7306: F, t7418: F, t7521: F, t1991: F, t2860: F, t1954: F, t2848: F, t723: F, t730: F, t1107: F, t5498: F, t1980: F, t1976: F, t2874: F) -> (F, F, F, F, F, F, F) {
    let t7524 = t237 * (t7266 + t7306 + t7418 + t7521);
    let t7526 = 0.11696447245269292414e1 * t2860 * t1991;
    let t7527 = t1954 * t2848;
    let t7528 = t7527 * t723;
    let t7530 = 0.23392894490538584828e1 * t730 * t7528;
    let t7531 = t5498 * t1107;
    let t7532 = t7531 * t1980;
    let t7534 = 0.10389515463408878255e3 * t730 * t7532;
    let t7535 = t1976 * t2848;
    let t7536 = t7535 * t2874;
    (t7524, t7526, t7528, t7530, t7532, t7534, t7536)
}
