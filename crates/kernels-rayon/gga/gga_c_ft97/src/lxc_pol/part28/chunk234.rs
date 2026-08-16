//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 234/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk234(t1774: f64, t9: f64, t466: f64, t458: f64, t471: f64, t1554: f64, t82: f64) -> (f64, f64, f64, f64) {
    let t1775 = t9 * t1774;
    let t1776 = t1775 * t466;
    let t1778 = t458 * t471;
    let t1780 = t1554 * t82;
    (t1775, t1776, t1778, t1780)
}
