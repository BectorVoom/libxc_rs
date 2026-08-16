//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 233/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk233(t1557: f64, t1736: f64, t1570: f64, t422: f64, t95: f64, t96: f64, t1542: f64, t9: f64, t94: f64, t17: f64, t351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1737 = t1736 * t1557;
    let t1742 = t422 * t1570;
    let t1766 = 1.0_f64 / t96 / t95;
    let t1771 = t9 * t1542;
    let t1773 = 4.0_f64 / 9.0_f64 * t1771 * t94;
    let t1774 = t351 * t17;
    (t1737, t1742, t1766, t1771, t1773, t1774)
}
