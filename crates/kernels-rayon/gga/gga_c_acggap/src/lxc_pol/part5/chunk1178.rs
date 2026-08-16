//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1178/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1178(t12589: f64, t5749: f64, t1131: f64, t1150: f64, t1173: f64, t1181: f64, t1532: f64, t16529: f64, t16533: f64, t16537: f64, t16542: f64, t1753: f64, t1879: f64, t3282: f64, t335: f64, t336: f64, t367: f64, t429: f64, t4838: f64, t540: f64, t5674: f64, t6308: f64, t6379: f64, t6383: f64, t839: f64, t960: f64) -> f64 {
    let t21361 = t12589 * t5749;
    let t21386 = -0.17149607247227894789e-2_f64 * t16529 + 0.85748036236139473944e-3_f64 * t16533 + 0.34299214494455789578e-2_f64 * t1173 * t1181 * t1532 * t1753 * t839 + 0.34299214494455789578e-2_f64 * t16537 - 0.68598428988911579156e-2_f64 * t21361 - 0.25724410870841842183e-2_f64 * t16542 - t367 * t336 * t429 * t5674 / 48.0_f64 + t367 * t960 * t540 * t4838 / 24.0_f64 - t1150 * t3282 * t6379 / 8.0_f64 + t1150 * t3282 * t6383 / 4.0_f64 - t335 * t960 * t6308 * t839 / 24.0_f64 - t367 * t960 * t1879 * t1131 / 16.0_f64;
    t21386
}
