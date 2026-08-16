//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1550/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1550(t16551: f64, t994: f64, t16558: f64, t16505: f64, t11627: f64, t42859: f64, t342: f64, t11631: f64, t43351: f64, t1024: f64, t1043: f64, t1082: f64, t1089: f64, t11788: f64, t11804: f64, t11940: f64, t12079: f64, t12086: f64, t12089: f64, t12111: f64, t12116: f64, t12119: f64, t12128: f64, t12149: f64, t12160: f64, t12168: f64, t3223: f64, t42001: f64, t42097: f64, t42615: f64, t43342: f64, t43348: f64, t4996: f64, t4998: f64) -> f64 {
    let t43520 = t994 * t16551;
    let t43524 = t994 * t16558;
    let t43528 = t994 * t16505;
    let t43536 = t42859 * t11627;
    let t43537 = t342 * t43536;
    let t43538 = t43351 * t11631;
    let t43558 = -0.15805078039045227836e2_f64 * t43520 * t43342 * t12168 + 0.15805078039045227836e2_f64 * t43524 * t43342 * t12079 + 0.79025390195226139183e1_f64 * t43528 * t12128 - 0.65854491829355115987e0_f64 * t1024 * t1082 * t42097 - 0.26341796731742046395e1_f64 * t3223 * t12111 - 0.23707617058567841754e2_f64 * t43537 * t43348 * t43538 + 0.15805078039045227836e2_f64 * t12116 * t12086 - 0.23707617058567841754e2_f64 * t11940 * t1082 * t42001 - 0.79025390195226139183e1_f64 * t12160 * t12089 - 0.26341796731742046395e1_f64 * t4996 * t42615 * t4998 + 0.15805078039045227836e2_f64 * t11788 * t12119 + 0.15805078039045227836e2_f64 * t12149 * t11804 * t1043 * t1089;
    t43558
}
