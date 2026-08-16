//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 813/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk813(t1557: f64, t469: f64, t26: f64, t356: f64, t1570: f64, t100: f64, t1587: f64, t1852: f64, t463: f64, t110: f64, t8216: f64, t8275: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11756 = t469 * t1557;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11810 = t1587 * t100;
    let t11854 = t463 * t1852;
    let t11863 = t8216 * t110;
    let t11987 = t8275 * t100;
    (t11756, t11761, t11762, t11810, t11854, t11863, t11987)
}
