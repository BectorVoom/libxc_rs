//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 433/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk433(t537: f64, t997: f64, t542: f64, t532: f64, t330: f64, t509: f64, t527: f64, t952: f64, t1173: f64, t1180: f64, t1542: f64, t1545: f64, t1549: f64, t1554: f64, t1558: f64, t1562: f64, t1565: f64, t1569: f64, t1572: f64, t1576: f64, t1581: f64, t1584: f64, t1588: f64, t418: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1591 = t997 * t537;
    let t1593 = t997 * t542;
    let t1595 = t997 * t532;
    let t1597 = t330 * t509;
    let t1599 = t952 * t527;
    let t1601 = 0.21437009059034868486e-3_f64 * t1542 + 0.85748036236139473944e-3_f64 * t1173 * t1545 - 0.42874018118069736972e-3_f64 * t1180 * t1549 + 0.42874018118069736972e-3_f64 * t1180 * t1554 - 0.21437009059034868486e-3_f64 * t1180 * t1558 + 0.85748036236139473944e-3_f64 * t1173 * t1562 - 0.85748036236139473944e-3_f64 * t1565 - 0.85748036236139473944e-3_f64 * t418 * t1569 + 0.42874018118069736972e-3_f64 * t1572 + 0.42874018118069736972e-3_f64 * t418 * t1576 + 0.42874018118069736972e-3_f64 * t418 * t1581 - 0.42874018118069736972e-3_f64 * t1584 - 0.42874018118069736972e-3_f64 * t418 * t1588 - 0.20007875121765877254e-2_f64 * t1591 + 0.20007875121765877254e-2_f64 * t1593 + 0.40015750243531754507e-2_f64 * t1595 - 7.0_f64 / 288.0_f64 * t1597 + 0.10003937560882938627e-2_f64 * t1599;
    (t1591, t1593, t1595, t1597, t1599, t1601)
}
