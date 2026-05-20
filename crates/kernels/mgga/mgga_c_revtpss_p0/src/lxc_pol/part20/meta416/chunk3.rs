//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1550/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1550<F: Float>(t16551: F, t994: F, t16558: F, t16505: F, t11627: F, t42859: F, t342: F, t11631: F, t43351: F, t1024: F, t1043: F, t1082: F, t1089: F, t11788: F, t11804: F, t11940: F, t12079: F, t12086: F, t12089: F, t12111: F, t12116: F, t12119: F, t12128: F, t12149: F, t12160: F, t12168: F, t3223: F, t42001: F, t42097: F, t42615: F, t43342: F, t43348: F, t4996: F, t4998: F) -> F {
    let t43520 = t994 * t16551;
    let t43524 = t994 * t16558;
    let t43528 = t994 * t16505;
    let t43536 = t42859 * t11627;
    let t43537 = t342 * t43536;
    let t43538 = t43351 * t11631;
    let t43558 = -F::cast_from(0.15805078039045227836e2_f64) * t43520 * t43342 * t12168 + F::cast_from(0.15805078039045227836e2_f64) * t43524 * t43342 * t12079 + F::cast_from(0.79025390195226139183e1_f64) * t43528 * t12128 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1082 * t42097 - F::cast_from(0.26341796731742046395e1_f64) * t3223 * t12111 - F::cast_from(0.23707617058567841754e2_f64) * t43537 * t43348 * t43538 + F::cast_from(0.15805078039045227836e2_f64) * t12116 * t12086 - F::cast_from(0.23707617058567841754e2_f64) * t11940 * t1082 * t42001 - F::cast_from(0.79025390195226139183e1_f64) * t12160 * t12089 - F::cast_from(0.26341796731742046395e1_f64) * t4996 * t42615 * t4998 + F::cast_from(0.15805078039045227836e2_f64) * t11788 * t12119 + F::cast_from(0.15805078039045227836e2_f64) * t12149 * t11804 * t1043 * t1089;
    t43558
}
