//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2979/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2979(t16052: f64, t16055: f64, t15752: f64, t16049: f64, t13392: f64, t4786: f64, t15599: f64, t4181: f64, t15968: f64, t1041: f64, t1042: f64, t1045: f64, t11268: f64, t15700: f64, t15701: f64, t16186: f64, t16226: f64, t1671: f64, t3124: f64, t373: f64, t42769: f64, t42772: f64, t42934: f64, t4869: f64, t54249: f64) -> (f64, f64, f64, f64) {
    let t54259 = t16052 * t16055;
    let t54261 = t16049 * t15752;
    let t54263 = t13392 * t4786;
    let t54267 = t4181 * t15599;
    let t54271 = t4181 * t15968;
    let t54275 = -0.85748036236139473944e-3_f64 * t42769 + 0.85748036236139473944e-3_f64 * t42772 + 0.64311027177104605458e-3_f64 * t3124 * t16186 + 0.21437009059034868486e-3_f64 * t1041 * t1042 * t373 * t54249 * t1045 + 0.21722835846488666732e-1_f64 * t42934 * t1671 + 0.21722835846488666732e-1_f64 * t11268 * t4869 - 0.91464571985215438873e-2_f64 * t54259 + 0.45732285992607719436e-2_f64 * t54261 - 0.85748036236139473944e-3_f64 * t15700 * t15701 * t54263 - 0.85748036236139473944e-3_f64 * t15700 * t15701 * t54267 - 0.17149607247227894789e-2_f64 * t16226 * t15701 * t54271;
    (t54263, t54267, t54271, t54275)
}
