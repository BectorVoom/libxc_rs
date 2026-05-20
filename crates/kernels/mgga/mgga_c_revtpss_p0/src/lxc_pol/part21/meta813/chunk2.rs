//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2979/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2979<F: Float>(t16052: F, t16055: F, t15752: F, t16049: F, t13392: F, t4786: F, t15599: F, t4181: F, t15968: F, t1041: F, t1042: F, t1045: F, t11268: F, t15700: F, t15701: F, t16186: F, t16226: F, t1671: F, t3124: F, t373: F, t42769: F, t42772: F, t42934: F, t4869: F, t54249: F) -> (F, F, F, F) {
    let t54259 = t16052 * t16055;
    let t54261 = t16049 * t15752;
    let t54263 = t13392 * t4786;
    let t54267 = t4181 * t15599;
    let t54271 = t4181 * t15968;
    let t54275 = -F::cast_from(0.85748036236139473944e-3_f64) * t42769 + F::cast_from(0.85748036236139473944e-3_f64) * t42772 + F::cast_from(0.64311027177104605458e-3_f64) * t3124 * t16186 + F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t1042 * t373 * t54249 * t1045 + F::cast_from(0.21722835846488666732e-1_f64) * t42934 * t1671 + F::cast_from(0.21722835846488666732e-1_f64) * t11268 * t4869 - F::cast_from(0.91464571985215438873e-2_f64) * t54259 + F::cast_from(0.45732285992607719436e-2_f64) * t54261 - F::cast_from(0.85748036236139473944e-3_f64) * t15700 * t15701 * t54263 - F::cast_from(0.85748036236139473944e-3_f64) * t15700 * t15701 * t54267 - F::cast_from(0.17149607247227894789e-2_f64) * t16226 * t15701 * t54271;
    (t54263, t54267, t54271, t54275)
}
