//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 360/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk360(t122: f64, t136: f64, t653: f64, t116: f64, t1033: f64, t190: f64, t1037: f64, t5: f64, t198: f64, t186: f64, t187: f64, t128: f64, t195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1620 = t136 * t122;
    let t1621 = t1620 * t653;
    let t1622 = t116 * t1621;
    let t1623 = t190 * t1033;
    let t1625 = t1037 * t5;
    let t1626 = t1623 * t198 * t1625;
    let t1629 = t136 * t186;
    let t1630 = t187 * t187;
    let t1631 = 1.0_f64 / t1630;
    let t1632 = t1629 * t1631;
    let t1633 = t116 * t1632;
    let t1636 = t128 * t195;
    (t1622, t1623, t1625, t1626, t1629, t1631, t1633, t1636)
}
