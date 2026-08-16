//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 440/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk440(t259: f64, t479: f64, t1578: f64, t1561: f64, t466: f64, t1141: f64, t1143: f64, t220: f64, t468: f64, t1139: f64, t1136: f64, t473: f64, t1153: f64, t1402: f64, t1507: f64, t1521: f64, t1547: f64, t1549: f64, t1553: f64, t198: f64, t330: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t480 = t259 < t479;
    let t1579 = param_beta * t1578;
    let t1581 = t466 * t1561;
    let t1586 = t1141 * t1143 * t1581 + t1578 * t220 * t468;
    let t1587 = t1139 * t1586;
    let t1589 = -t1136 * t1587 + t1579 * t473;
    let t1594 = piecewise3(t480, t1153 * t1589 * t198 * t330 - t1507 + t1521 + t1547 + t1549 - t1553, t1402);
    (t1579, t1581, t1586, t1587, t1589, t1594)
}
