//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 439/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk439(t1539: f64, t1629: f64, t1160: f64, t1533: f64, t1251: f64, t525: f64, t1411: f64, t456: f64, t407: f64, t310: f64, t553: f64, t159: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1630 = t1629 * t1539;
    let t1631 = t1160 * t1630;
    let t1633 = t1629 * t1533;
    let t1636 = t1251 * t525;
    let t1639 = t456 * t1411;
    let t1642 = t1629 * t407;
    let t1645 = t310 * t553;
    let t1647 = t159 * t545;
    (t1630, t1631, t1633, t1636, t1639, t1642, t1645, t1647)
}
