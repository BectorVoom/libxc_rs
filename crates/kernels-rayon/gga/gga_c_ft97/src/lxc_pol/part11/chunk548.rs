//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 548/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk548(t364: f64, t7773: f64, t89: f64, t1546: f64, t1581: f64, t1554: f64, t375: f64, t1560: f64, t1558: f64, t7765: f64, t356: f64, t1570: f64, t363: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7775 = t89 * t7773 * t364;
    let t7778 = t89 * t1546 * t1581;
    let t7780 = t375 * t1554;
    let t7782 = t89 * t7780 * t1560;
    let t7784 = t1558 * t7765;
    let t7786 = t89 * t356 * t7784;
    let t7788 = t1570 * t363;
    (t7775, t7778, t7780, t7782, t7784, t7786, t7788)
}
