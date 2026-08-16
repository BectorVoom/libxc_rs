//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1544/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1544(t42872: f64, t43351: f64, t1086: f64, t3259: f64, t994: f64, t3046: f64, t4980: f64, t11249: f64, t3133: f64, t1083: f64, t1089: f64, t11202: f64, t11782: f64, t11940: f64, t12052: f64, t12124: f64, t12149: f64, t16559: f64, t16561: f64, t16566: f64, t16568: f64, t3059: f64, t3288: f64, t3291: f64, t3292: f64, t3317: f64, t3318: f64, t42278: f64, t43334: f64, t43341: f64, t43342: f64, t43347: f64, t43348: f64) -> f64 {
    let t43352 = t43351 * t42872;
    let t43357 = t994 * t1086 * t3259;
    let t43360 = t3046 * t4980;
    let t43367 = t3133 * t11249;
    let t43374 = -0.26341796731742046395e1_f64 * t42278 * t1083 - 0.79025390195226139183e1_f64 * t11782 * t3292 - 0.19756347548806534796e1_f64 * t3317 * t43334 * t3318 - 0.15805078039045227836e2_f64 * t11940 * t3291 * t11202 - 0.26341796731742046395e1_f64 * t43341 * t43342 * t12052 + 0.15805078039045227836e2_f64 * t43347 * t43348 * t43352 - 0.79025390195226139183e1_f64 * t43357 * t3288 - 0.15805078039045227836e2_f64 * t43360 * t12124 + 0.79025390195226139183e1_f64 * t12149 * t3059 * t3133 * t1089 - 0.23707617058567841754e2_f64 * t16559 * t43367 * t16561 + 0.39512695097613069592e1_f64 * t16566 * t43367 * t16568;
    t43374
}
