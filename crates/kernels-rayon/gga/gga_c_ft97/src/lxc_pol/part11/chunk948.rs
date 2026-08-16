//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 948/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk948(t1882: f64, t9038: f64, t9051: f64, t9079: f64, t1642: f64, t1984: f64, t1643: f64, t1986: f64, t446: f64, t558: f64, t7959: f64, t9049: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39687 = t1882 * t9038;
    let t39689 = t1882 * t9051;
    let t39691 = t1882 * t9079;
    let t39693 = t1642 * t1984;
    let t39694 = t1643 * t1986;
    let t39696 = t446 * t39693 * t39694;
    let t39698 = t7959 * t558;
    let t39700 = t446 * t9049 * t39698;
    (t39687, t39689, t39691, t39694, t39696, t39698, t39700)
}
