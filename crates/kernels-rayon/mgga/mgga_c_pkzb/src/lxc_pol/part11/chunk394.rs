//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 394/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk394(t114: f64, t1613: f64, t1504: f64, t541: f64, t1497: f64, t1503: f64, t1507: f64) -> (f64, f64, f64, f64, f64) {
    let t1614 = t114 * t1613;
    let t1615 = t1504 * t541;
    let t1618 = t1497 * t541;
    let t1621 = t114 * t1503;
    let t1622 = t1504 * t1507;
    (t1614, t1615, t1618, t1621, t1622)
}
