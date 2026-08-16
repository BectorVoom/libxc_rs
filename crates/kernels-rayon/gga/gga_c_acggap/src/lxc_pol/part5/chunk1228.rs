//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1228/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1228(t1101: f64, t1165: f64, t17888: f64, t1884: f64, t1008: f64, t5539: f64, t322: f64, t368: f64, t384: f64, t398: f64, t5784: f64, t1089: f64, t1095: f64, t13545: f64, t17281: f64, t17291: f64, t17302: f64, t17304: f64, t17306: f64, t1839: f64, t301: f64, t3201: f64, t397: f64, t418: f64, t5674: f64, t6074: f64, t966: f64) -> f64 {
    let t22470 = t17888 * t1165 * t1884 * t1101;
    let t22473 = t1008 * t5539;
    let t22488 = t384 * t398 * t368 * t5784 * t322;
    let t22492 = 0.17149607247227894789e-2_f64 * t13545 + 0.85748036236139473944e-3_f64 * t17281 - 0.42874018118069736972e-3_f64 * t397 * t398 * t966 * t1839 + 0.51448821741683684366e-1_f64 * t22470 + 0.17149607247227894789e-2_f64 * t17291 + 0.51448821741683684368e-2_f64 * t22473 + 0.34299214494455789578e-2_f64 * t418 * t1089 * t1095 * t5674 * t301 - 0.90702367218671976884e-1_f64 * t17302 - 0.17149607247227894789e-2_f64 * t418 * t398 * t3201 * t6074 - 0.85748036236139473944e-3_f64 * t22488 - 0.34299214494455789578e-2_f64 * t17304 - 0.34299214494455789577e-2_f64 * t17306;
    t22492
}
