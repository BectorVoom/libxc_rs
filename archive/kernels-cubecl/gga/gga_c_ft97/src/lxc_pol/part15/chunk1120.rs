//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1120/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1120<F: Float>(t200: F, t88503: F, t1111: F, t1115: F, t1127: F, t13443: F, t13582: F, t13586: F, t1701: F, t17825: F, t17964: F, t17965: F, t17975: F, t17987: F, t18090: F, t2035: F, t21159: F, t21227: F, t21239: F, t21264: F, t21292: F, t21331: F, t21332: F, t22090: F, t2387: F, t30651: F, t30688: F, t41768: F, t4978: F, t4979: F, t5003: F, t5007: F, t5025: F, t66313: F, t66318: F, t6757: F, t678: F, t680: F, t79629: F, t79956: F, t79964: F, t80157: F, t807: F, t88384: F, t88405: F, t88493: F, t9533: F) -> (F, F) {
    let t88504 = t88503 * t200;
    let t88536 = F::cast_from(0.16540877980489188955e-3_f64) * t17825 * t88405 + F::cast_from(0.19608816007975193346e-4_f64) * t79964 * t21331 * t21332 * t79629 - F::cast_from(0.5509824679191440163e-4_f64) * t79956 * t88384 - F::cast_from(0.139529405678626752e0_f64) * t9533 * t680 * t4978 * t5025 + F::cast_from(0.12901581267952785412e-4_f64) * t2387 * t807 * t88493 - F::cast_from(0.139529405678626752e0_f64) * t18090 * t4979 - F::cast_from(0.32447425081717998846e-3_f64) * t80157 * t13582 + F::cast_from(0.16223712540858999423e-3_f64) * t80157 * t13586 + F::cast_from(0.53719526674014200183e-7_f64) * t678 * t41768 * t88504 - F::cast_from(0.14225713554822031171e0_f64) * t21159 * t1111 - F::cast_from(0.558117622714507008e0_f64) * t66318 * t6757 * t17965 * t5025 - F::cast_from(0.558117622714507008e-1_f64) * t17964 * t30651 * t21292 - F::cast_from(0.1422571355482203117e0_f64) * t21227 * t1111 + F::cast_from(0.474190451827401039e-1_f64) * t21239 * t1111 - F::cast_from(0.45048092923603098704e0_f64) * t13443 * t1701 * t17975 * t5025 + F::cast_from(0.10680687768703050405e-1_f64) * t17987 * t2035 * t22090 * t1127 - F::cast_from(0.45048092923603098705e0_f64) * t5007 * t5003 - F::cast_from(0.2136137553740610081e-1_f64) * t1115 * t21264 - F::cast_from(0.46477736175058559857e-2_f64) * t66313 * t30688 * t21292;
    (t88504, t88536)
}
