//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 408/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk408(t110: f64, t123: f64, t1485: f64, t1520: f64, t1530: f64, t1553: f64, t1564: f64, t1570: f64, t1572: f64, t1582: f64, t1587: f64, t1590: f64, t1596: f64, t1604: f64, t1608: f64, t1614: f64, t1615: f64, t1618: f64, t1621: f64, t1622: f64, t204: f64, t49: f64, t520: f64, t527: f64, t535: f64, t542: f64) -> f64 {
    let t1625 = -0.70983522622222222221e-3_f64 * t49 * t1485 * t110 - 0.34246666666666666666e-1_f64 * t204 * t1564 * t527 - 2.0_f64 * t1570 * t1572 + 1.0_f64 * t520 * t1582 + 0.32163958997385070134e2_f64 * t1587 * t1590 + t1553 + t1596 + t1520 - t1530 - t1604 - 0.24415263074675393405e-3_f64 * t49 * t1485 * t123 - 0.10843581300301739842e-1_f64 * t204 * t1608 * t542 - 0.11696447245269292414e1_f64 * t1614 * t1615 + 0.5848223622634646207e0_f64 * t535 * t1618 + 0.17315859105681463759e2_f64 * t1621 * t1622;
    t1625
}
