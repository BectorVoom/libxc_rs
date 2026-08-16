//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1249/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1249(t1111: f64, t1165: f64, t21532: f64, t4282: f64, t1096: f64, t1181: f64, t16871: f64, t17592: f64, t17605: f64, t17607: f64, t17613: f64, t17615: f64, t17617: f64, t18751: f64, t20206: f64, t3396: f64, t4199: f64, t4267: f64, t4450: f64, t4463: f64, t4526: f64, t5258: f64, t530: f64, t5852: f64) -> f64 {
    let t22962 = t4282 * t1165 * t21532 * t1111;
    let t22985 = 0.17149607247227894789e-2_f64 * t17592 + 0.34299214494455789578e-1_f64 * t4463 * t1181 * t530 * t5258 + 0.17149607247227894789e-1_f64 * t22962 + 0.85748036236139473944e-3_f64 * t17605 + 0.68598428988911579156e-2_f64 * t3396 * t1181 * t4267 * t4526 + 0.10289764348336736873e0_f64 * t16871 * t1165 * t21532 * t1096 + 0.51448821741683684368e-2_f64 * t18751 * t1165 * t5852 * t20206 + 0.85748036236139473944e-3_f64 * t17607 - 0.77173232612525526552e-2_f64 * t4450 * t1165 * t5852 * t4199 + 0.80031500487063509016e-2_f64 * t17613 + 0.17149607247227894789e-2_f64 * t17615 + 0.17149607247227894789e-2_f64 * t17617;
    t22985
}
