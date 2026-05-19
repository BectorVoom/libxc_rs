//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 395/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk395<F: Float>(t110: F, t123: F, t1485: F, t1520: F, t1530: F, t1553: F, t1564: F, t1570: F, t1572: F, t1582: F, t1587: F, t1590: F, t1596: F, t1604: F, t1608: F, t1614: F, t1615: F, t1618: F, t1621: F, t1622: F, t204: F, t49: F, t520: F, t527: F, t535: F, t542: F) -> F {
    let t1625 = -F::cast_from(0.70983522622222222221e-3_f64) * t49 * t1485 * t110 - F::cast_from(0.34246666666666666666e-1_f64) * t204 * t1564 * t527 - F::new(2.0) * t1570 * t1572 + F::new(1.0) * t520 * t1582 + F::cast_from(0.32163958997385070134e2_f64) * t1587 * t1590 + t1553 + t1596 + t1520 - t1530 - t1604 - F::cast_from(0.24415263074675393405e-3_f64) * t49 * t1485 * t123 - F::cast_from(0.10843581300301739842e-1_f64) * t204 * t1608 * t542 - F::cast_from(0.11696447245269292414e1_f64) * t1614 * t1615 + F::cast_from(0.5848223622634646207e0_f64) * t535 * t1618 + F::cast_from(0.17315859105681463759e2_f64) * t1621 * t1622;
    t1625
}
