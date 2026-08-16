//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 656/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk656(t157: f64, t9114: f64, t1559: f64, t558: f64, t3440: f64, t1557: f64, t604: f64, t609: f64, t3439: f64, t1570: f64, t2210: f64, t1984: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9115 = t9114 * t157;
    let t9116 = t1559 * t558;
    let t9117 = t3440 * t9116;
    let t9118 = t9115 * t9117;
    let t9121 = t604 * t1557;
    let t9122 = t1559 * t609;
    let t9123 = t9121 * t9122;
    let t9124 = t3439 * t9123;
    let t9127 = t604 * t1570;
    let t9128 = t9127 * t9122;
    let t9129 = t2210 * t9128;
    let t9132 = t355 * t1984;
    (t9115, t9116, t9117, t9118, t9121, t9123, t9124, t9127, t9128, t9129, t9132)
}
