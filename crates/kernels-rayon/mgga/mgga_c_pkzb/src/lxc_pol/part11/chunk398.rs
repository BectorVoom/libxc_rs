//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 398/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk398(t1628: f64, t83: f64, t501: f64, t513: f64, t142: f64, t192: f64, t1504: f64, t1613: f64, t541: f64) -> (f64, f64, f64, f64) {
    let t1629 = t83 * t1628;
    let t1631 = t501 * t513;
    let t1633 = t142 * t192;
    let t1639 = t1613 * t1504 * t541;
    (t1629, t1631, t1633, t1639)
}
