//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 663/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk663(t1775: f64, t2109: f64, t2098: f64, t2114: f64, t458: f64, t582: f64, t8307: f64, t3506: f64, t7789: f64, t2: f64, t9132: f64, t9074: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9205 = t1775 * t2109;
    let t9207 = t1775 * t2098;
    let t9209 = t458 * t2114;
    let t9211 = t582 * t8307;
    let t9214 = t3506 * t7789;
    let t9217 = t9132 * t2;
    let t9218 = t9217 * t9074;
    (t9205, t9207, t9209, t9211, t9214, t9217, t9218)
}
