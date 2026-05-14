//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 344/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk344<F: Float>(t537: F, t997: F, t542: F, t532: F, t330: F, t509: F, t527: F, t952: F, t1173: F, t1180: F, t1542: F, t1545: F, t1549: F, t1554: F, t1558: F, t1562: F, t1565: F, t1569: F, t1572: F, t1576: F, t1581: F, t1584: F, t1588: F, t418: F) -> (F, F, F, F, F, F) {
    let t1591 = t997 * t537;
    let t1593 = t997 * t542;
    let t1595 = t997 * t532;
    let t1597 = t330 * t509;
    let t1599 = t952 * t527;
    let t1601 = 0.21437009059034868486e-3 * t1542 + 0.85748036236139473944e-3 * t1173 * t1545 - 0.42874018118069736972e-3 * t1180 * t1549 + 0.42874018118069736972e-3 * t1180 * t1554 - 0.21437009059034868486e-3 * t1180 * t1558 + 0.85748036236139473944e-3 * t1173 * t1562 - 0.85748036236139473944e-3 * t1565 - 0.85748036236139473944e-3 * t418 * t1569 + 0.42874018118069736972e-3 * t1572 + 0.42874018118069736972e-3 * t418 * t1576 + 0.42874018118069736972e-3 * t418 * t1581 - 0.42874018118069736972e-3 * t1584 - 0.42874018118069736972e-3 * t418 * t1588 - 0.20007875121765877254e-2 * t1591 + 0.20007875121765877254e-2 * t1593 + 0.40015750243531754507e-2 * t1595 - 7.0 / 288.0 * t1597 + 0.10003937560882938627e-2 * t1599;
    (t1591, t1593, t1595, t1597, t1599, t1601)
}
