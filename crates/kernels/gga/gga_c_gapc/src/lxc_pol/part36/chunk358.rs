//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 358/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk358<F: Float>(t1431: F, t1689: F, t1688: F, t190: F, t644: F, t640: F, t633: F, t198: F, t442: F, t457: F, t6: F, t1037: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1690 = t1689 * t1431;
    let t1691 = t1688 * t1690;
    let t1694 = t190 * t644;
    let t1695 = t640 * t1694;
    let t1696 = t633 * t1695;
    let t1697 = t442 * t198;
    let t1698 = t457 * t6;
    let t1699 = t1037 * t1698;
    let t1700 = t1697 * t1699;
    (t1690, t1691, t1694, t1695, t1696, t1697, t1698, t1699, t1700)
}
