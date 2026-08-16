//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1226/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1226(t5878: f64, t997: f64, t1084: f64, t1180: f64, t1181: f64, t13503: f64, t17221: f64, t17223: f64, t17228: f64, t17230: f64, t17232: f64, t17234: f64, t17236: f64, t17238: f64, t20764: f64, t4289: f64, t4735: f64, t5799: f64) -> f64 {
    let t22431 = t997 * t5878;
    let t22445 = 0.17149607247227894789e-2_f64 * t17221 + 0.51448821741683684366e-2_f64 * t17223 + 0.85748036236139473944e-3_f64 * t17228 - 0.34299214494455789578e-2_f64 * t17230 + 0.34299214494455789577e-2_f64 * t17232 + 0.16006300097412701803e-1_f64 * t22431 + 0.68598428988911579156e-2_f64 * t17234 + 0.12004725073059526352e-1_f64 * t17236 - 0.34299214494455789578e-2_f64 * t17238 - 0.17149607247227894789e-2_f64 * t1180 * t1181 * t4289 * t5799 + 0.51448821741683684367e-2_f64 * t4735 * t1181 * t20764 * t1084 + 0.24009450146119052705e-1_f64 * t13503;
    t22445
}
