//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1544/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1544<F: Float>(t42872: F, t43351: F, t1086: F, t3259: F, t994: F, t3046: F, t4980: F, t11249: F, t3133: F, t1083: F, t1089: F, t11202: F, t11782: F, t11940: F, t12052: F, t12124: F, t12149: F, t16559: F, t16561: F, t16566: F, t16568: F, t3059: F, t3288: F, t3291: F, t3292: F, t3317: F, t3318: F, t42278: F, t43334: F, t43341: F, t43342: F, t43347: F, t43348: F) -> F {
    let t43352 = t43351 * t42872;
    let t43357 = t994 * t1086 * t3259;
    let t43360 = t3046 * t4980;
    let t43367 = t3133 * t11249;
    let t43374 = -F::cast_from(0.26341796731742046395e1_f64) * t42278 * t1083 - F::cast_from(0.79025390195226139183e1_f64) * t11782 * t3292 - F::cast_from(0.19756347548806534796e1_f64) * t3317 * t43334 * t3318 - F::cast_from(0.15805078039045227836e2_f64) * t11940 * t3291 * t11202 - F::cast_from(0.26341796731742046395e1_f64) * t43341 * t43342 * t12052 + F::cast_from(0.15805078039045227836e2_f64) * t43347 * t43348 * t43352 - F::cast_from(0.79025390195226139183e1_f64) * t43357 * t3288 - F::cast_from(0.15805078039045227836e2_f64) * t43360 * t12124 + F::cast_from(0.79025390195226139183e1_f64) * t12149 * t3059 * t3133 * t1089 - F::cast_from(0.23707617058567841754e2_f64) * t16559 * t43367 * t16561 + F::cast_from(0.39512695097613069592e1_f64) * t16566 * t43367 * t16568;
    t43374
}
