//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1269/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1269(t1165: f64, t1552: f64, t1759: f64, t3451: f64, t864: f64, t3379: f64, t6271: f64, t1487: f64, t407: f64, t1173: f64, t1180: f64, t1181: f64, t1532: f64, t18027: f64, t18031: f64, t18035: f64, t18037: f64, t18041: f64, t18045: f64, t18047: f64, t301: f64, t372: f64, t5799: f64) -> (f64, f64) {
    let t23429 = t3451 * t1165 * t1552 * t1759 * t864;
    let t23431 = t3379 * t6271;
    let t23445 = t407 * t1487;
    let t23450 = -0.34299214494455789578e-2_f64 * t18027 - 0.34299214494455789578e-1_f64 * t18031 - 0.17149607247227894789e-2_f64 * t18035 + 0.32012600194825403606e-1_f64 * t18037 - 0.85748036236139473944e-3_f64 * t18041 + 0.17149607247227894789e-2_f64 * t23429 + 0.68598428988911579156e-2_f64 * t23431 - 0.85748036236139473944e-3_f64 * t18045 + 0.16006300097412701803e-1_f64 * t18047 + 0.68598428988911579156e-2_f64 * t1173 * t1181 * t1532 * t5799 * t301 + 0.34299214494455789578e-2_f64 * t1180 * t1181 * t1552 * t5799 * t372 - 0.17149607247227894789e-2_f64 * t1180 * t1181 * t1532 * t23445;
    (t23445, t23450)
}
