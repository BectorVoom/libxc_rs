//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1334/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1334<F: Float>(t10296: F, t10361: F, t10362: F, t11493: F, t11494: F, t11497: F, t11532: F, t11533: F, t1246: F, t19227: F, t23398: F, t2428: F, t32261: F, t32337: F, t32395: F, t3254: F, t3278: F, t3928: F, t411: F, t8500: F, t938: F, t942: F, t951: F) -> F {
    let t32400 = -F::cast_from(0.19756347548806534796e1_f64) * t1246 * t10362 - F::cast_from(0.39512695097613069591e1_f64) * t938 * t11494 + F::cast_from(0.15805078039045227836e2_f64) * t411 * t19227 * t11493 * t951 - F::cast_from(0.11853808529283920877e2_f64) * t411 * t10296 * t3278 + F::cast_from(0.39512695097613069591e1_f64) * t938 * t11497 - F::cast_from(0.11853808529283920877e2_f64) * t23398 * t32261 * t951 + F::cast_from(0.39512695097613069591e1_f64) * t411 * t8500 * t3928 + F::cast_from(0.39512695097613069591e1_f64) * t411 * t3254 * t10361 - F::cast_from(0.65854491829355115987e0_f64) * t938 * t11533 + F::cast_from(0.13170898365871023197e1_f64) * t411 * t2428 * t11532 * t951 - F::cast_from(0.65854491829355115987e0_f64) * t411 * t942 * (t32337 + t32395);
    t32400
}
