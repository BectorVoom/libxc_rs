//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 383/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk383(t173: f64, t1775: f64, t1764: f64, t204: f64, t505: f64, t200: f64, t203: f64, t197: f64, t617: f64, t663: f64, t126: f64, t145: f64) -> (f64, f64, f64, f64, f64, f64) {
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
