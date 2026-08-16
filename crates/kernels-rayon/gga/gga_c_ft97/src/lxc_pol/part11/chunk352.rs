//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 352/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk352(t1565: f64, t1787: f64, t1570: f64, t2: f64, t1559: f64, t463: f64, t1580: f64, t464: f64, t1586: f64, t1588: f64, t24: f64, t1755: f64, t469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1788 = t1787 * t1565;
    let t1791 = t2 * t1570;
    let t1792 = t1791 * t1559;
    let t1793 = t463 * t1792;
    let t1796 = t464 * t1580;
    let t1797 = t463 * t1796;
    let t1800 = t1586 * t2;
    let t1802 = t24 * t1800 * t1588;
    let t1806 = t24 * t469 * t1755;
    (t1788, t1791, t1792, t1793, t1796, t1797, t1800, t1802, t1806)
}
