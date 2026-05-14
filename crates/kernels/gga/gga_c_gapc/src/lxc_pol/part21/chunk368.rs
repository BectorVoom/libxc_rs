//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 368/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk368<F: Float>(t1: F, t102: F, t1762: F, t619: F, t1686: F, t185: F, t505: F, t567: F, t1036: F, t1689: F, t147: F, t19: F, t995: F, t173: F, t204: F, t200: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1764 = t1762 * t1 * t102;
    let t1765 = t1764 * t619;
    let t1768 = t185 * t1686;
    let t1769 = t505 * t567;
    let t1771 = t1036 * t1689 * t1769;
    let t1775 = t995 * t19 * t147;
    let t1776 = t1775 * t173;
    let t1781 = t1764 * t204;
    let t1784 = t505 * t505;
    let t1785 = t1784 * t200;
    (t1764, t1765, t1768, t1771, t1775, t1776, t1781, t1784, t1785)
}
