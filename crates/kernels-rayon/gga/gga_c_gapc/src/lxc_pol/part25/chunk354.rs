//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 354/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk354(t1587: f64, t436: f64, t464: f64, t640: f64, t463: f64, t1: f64, t203: f64, t3: f64, t567: f64, t1417: f64, t126: f64, t516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1588 = t436 * t1587;
    let t1591 = t464 * t640;
    let t1592 = t463 * t1591;
    let t1593 = t203 * t1;
    let t1595 = t1593 * t3 * t567;
    let t1596 = t1417 * t1595;
    let t1599 = t516 * t126;
    (t1588, t1592, t1593, t1595, t1596, t1599)
}
