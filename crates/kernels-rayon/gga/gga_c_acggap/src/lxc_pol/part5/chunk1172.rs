//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1172/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1172(t13293: f64, t15386: f64, t21118: f64, t525: f64, t3621: f64, t6380: f64, t6384: f64, t1083: f64, t1165: f64, t1459: f64, t1531: f64, t16388: f64, t16390: f64, t16392: f64, t16398: f64, t336: f64, t360: f64, t3616: f64, t367: f64, t372: f64, t398: f64, t418: f64, t4838: f64, t5141: f64, t535: f64, t5674: f64, t5867: f64, t6374: f64, t839: f64, t960: f64) -> f64 {
    let t21189 = t13293 * t15386 * t525 * t21118;
    let t21209 = t3621 * t6380;
    let t21211 = t3621 * t6384;
    let t21217 = -0.17149607247227894789e-2_f64 * t1531 * t1165 * t5867 * t5141 + 0.51448821741683684366e-2_f64 * t21189 - t367 * t336 * t535 * t4838 / 48.0_f64 - 0.12004725073059526352e-1_f64 * t16388 - 0.90702367218671976884e-1_f64 * t16390 - 0.85748036236139473944e-3_f64 * t16392 - 0.22675591804667994222e-1_f64 * t16398 - 0.17149607247227894789e-2_f64 * t418 * t398 * t1083 * t5674 * t360 + 0.25724410870841842184e-2_f64 * t418 * t398 * t1459 * t5674 * t372 + 7.0_f64 / 24.0_f64 * t21209 - 7.0_f64 / 12.0_f64 * t21211 - t3616 * t960 * t6374 * t839 / 4.0_f64;
    t21217
}
