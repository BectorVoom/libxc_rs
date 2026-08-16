//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 996/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk996(t2187: f64, t8232: f64, t1882: f64, t9260: f64, t3281: f64, t576: f64, t611: f64, t558: f64, t7765: f64, t8392: f64, t9345: f64, t1559: f64, t1986: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40685 = t8232 * t2187;
    let t40690 = t1882 * t9260;
    let t40696 = t3281 * t576;
    let t40698 = t3281 * t611;
    let t40700 = t7765 * t558;
    let t40720 = t8392 * t9345;
    let t40722 = t1559 * t1986;
    (t40685, t40690, t40696, t40698, t40700, t40720, t40722)
}
