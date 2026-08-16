//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2266/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2266(t41008: f64, t4155: f64, t13076: f64, t9638: f64, t13322: f64, t13316: f64, t41115: f64, t4240: f64, t13278: f64, t2686: f64, t13173: f64, t2639: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46911 = t41008 * t4155;
    let t46912 = 35.0_f64 / 24.0_f64 * t46911;
    let t46918 = t9638 * t13076;
    let t46920 = t9638 * t13322;
    let t46926 = t9638 * t13316;
    let t46928 = t41115 * t4240;
    let t46929 = 119.0_f64 / 4608.0_f64 * t46928;
    let t46930 = t13278 * t2686;
    let t46936 = t2639 * t13173;
    (t46912, t46918, t46920, t46926, t46929, t46930, t46936)
}
