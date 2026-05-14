//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 362/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk362<F: Float>(t1694: F, t654: F, t633: F, t118: F, t1393: F, t129: F, t1303: F, t172: F, t153: F, t181: F, t1695: F, t185: F, t567: F, t6: F, t1689: F, t1697: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1703 = t654 * t1694;
    let t1704 = t633 * t1703;
    let t1707 = t1393 * t118;
    let t1708 = t1707 * t129;
    let t1709 = t172 * t1303;
    let t1710 = t153 * t1709;
    let t1711 = t181 * t1710;
    let t1714 = t185 * t1695;
    let t1715 = t6 * t567;
    let t1716 = t1689 * t1715;
    let t1717 = t1697 * t1716;
    (t1703, t1704, t1707, t1708, t1709, t1711, t1714, t1715, t1716, t1717)
}
