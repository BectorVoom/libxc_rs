//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1334/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1334(t10296: f64, t10361: f64, t10362: f64, t11493: f64, t11494: f64, t11497: f64, t11532: f64, t11533: f64, t1246: f64, t19227: f64, t23398: f64, t2428: f64, t32261: f64, t32337: f64, t32395: f64, t3254: f64, t3278: f64, t3928: f64, t411: f64, t8500: f64, t938: f64, t942: f64, t951: f64) -> f64 {
    let t32400 = -0.19756347548806534796e1_f64 * t1246 * t10362 - 0.39512695097613069591e1_f64 * t938 * t11494 + 0.15805078039045227836e2_f64 * t411 * t19227 * t11493 * t951 - 0.11853808529283920877e2_f64 * t411 * t10296 * t3278 + 0.39512695097613069591e1_f64 * t938 * t11497 - 0.11853808529283920877e2_f64 * t23398 * t32261 * t951 + 0.39512695097613069591e1_f64 * t411 * t8500 * t3928 + 0.39512695097613069591e1_f64 * t411 * t3254 * t10361 - 0.65854491829355115987e0_f64 * t938 * t11533 + 0.13170898365871023197e1_f64 * t411 * t2428 * t11532 * t951 - 0.65854491829355115987e0_f64 * t411 * t942 * (t32337 + t32395);
    t32400
}
