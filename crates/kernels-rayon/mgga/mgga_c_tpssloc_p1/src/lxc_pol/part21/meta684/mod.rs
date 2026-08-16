//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2497;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta684(t133: f64, t1484: f64, t41214: f64, t6600: f64, t12998: f64, t46766: f64, t686: f64, t776: f64, t12984: f64, t2553: f64, t12990: f64, t13012: f64, t12994: f64, t213: f64, t221: f64, t13004: f64, t782: f64, t13007: f64, t131: f64, t205: f64, t41160: f64, t116: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46806, t46819, t46828, t46830) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2497(t133, t1484, t41214, t6600, t12998, t46766, t686, t776, t12984, t2553, t12990, t13012);
        let (t46836, t46838, t46843, t46844, t46847, t46853) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2498(t12994, t13012, t213, t221, t13004, t782, t13007, t131, t205, t41160, t116, t212);
    (t46806, t46819, t46828, t46830, t46836, t46838, t46843, t46844, t46847, t46853)
}
