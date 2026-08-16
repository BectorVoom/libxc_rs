//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 358/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk358(t1630: f64, t1629: f64, t116: f64, t128: f64, t195: f64, t144: f64, t599: f64, t122: f64, t125: f64, t1457: f64, t169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1631 = 1.0_f64 / t1630;
    let t1632 = t1629 * t1631;
    let t1633 = t116 * t1632;
    let t1636 = t128 * t195;
    let t1638 = t1636 * t144 * t599;
    let t1642 = t1457 * t122 * t125;
    let t1643 = t169 * t1642;
    (t1631, t1633, t1636, t1638, t1642, t1643)
}
