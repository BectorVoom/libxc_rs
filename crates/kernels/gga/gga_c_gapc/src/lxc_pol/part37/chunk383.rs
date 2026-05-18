//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 383/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk383<F: Float>(t173: F, t1775: F, t1764: F, t204: F, t505: F, t200: F, t203: F, t197: F, t617: F, t663: F, t126: F, t145: F) -> (F, F, F, F, F, F) {
    let t1776 = t1775 * t173;
    let t1781 = t1764 * t204;
    let t1784 = t505 * t505;
    let t1785 = t1784 * t200;
    let t1786 = t1785 * t203;
    let t1787 = t197 * t1786;
    let t1790 = t617 * t663;
    let t1793 = t126 * t145;
    (t1776, t1781, t1784, t1787, t1790, t1793)
}
