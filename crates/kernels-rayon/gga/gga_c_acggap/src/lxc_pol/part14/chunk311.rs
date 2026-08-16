//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 311/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk311(t1434: f64, t384: f64, t1013: f64, t1034: f64, t1041: f64, t1044: f64, t1104: f64, t1109: f64, t1114: f64, t1138: f64, t1141: f64, t1168: f64, t1347: f64, t1353: f64, t1355: f64, t1413: f64, t1418: f64, t1424: f64, t1429: f64, t397: f64, t418: f64) -> (f64, f64) {
    let t1435 = t384 * t1434;
    let t1437 = -0.42874018118069736972e-3_f64 * t1013 + t1034 + t1041 - 7.0_f64 / 288.0_f64 * t1044 + 0.85748036236139473944e-3_f64 * t1104 - 0.42874018118069736972e-3_f64 * t1109 + 0.42874018118069736972e-3_f64 * t1114 + 7.0_f64 / 144.0_f64 * t1138 + 7.0_f64 / 288.0_f64 * t1141 + 0.21437009059034868486e-3_f64 * t1168 + 0.17149607247227894789e-2_f64 * t418 * t1347 - 0.42874018118069736972e-3_f64 * t1353 - 0.21437009059034868486e-3_f64 * t1355 - 0.21437009059034868486e-3_f64 * t397 * t1413 - 0.17149607247227894789e-2_f64 * t418 * t1418 + 0.85748036236139473944e-3_f64 * t1424 + 0.42874018118069736972e-2_f64 * t418 * t1429 + 0.42874018118069736972e-3_f64 * t1435;
    (t1435, t1437)
}
