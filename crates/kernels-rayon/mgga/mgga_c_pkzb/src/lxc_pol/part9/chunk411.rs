//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 411/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk411(t1628: f64, t83: f64, t501: f64, t513: f64, t142: f64, t192: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1629 = t83 * t1628;
    let t1630 = 2.0_f64 * t1629;
    let t1631 = t501 * t513;
    let t1632 = 8.0_f64 * t1631;
    let t1633 = t142 * t192;
    let t1634 = t568 * t568;
    (t1629, t1630, t1631, t1632, t1633, t1634)
}
