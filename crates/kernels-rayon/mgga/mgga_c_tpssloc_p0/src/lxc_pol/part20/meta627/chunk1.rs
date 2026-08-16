//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2267/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2267(t13186: f64, t13242: f64, t16836: f64, t2623: f64, t2643: f64, t41084: f64, t41086: f64, t41088: f64, t4167: f64, t46912: f64, t46918: f64, t46920: f64, t46926: f64, t46929: f64, t46930: f64, t46936: f64, t9634: f64, t9646: f64, t9647: f64, t9663: f64) -> f64 {
    let t46938 = t46912 + 455.0_f64 / 216.0_f64 * t41084 - 35.0_f64 / 72.0_f64 * t41086 + 7.0_f64 / 144.0_f64 * t41088 + t16836 * t9634 / 512.0_f64 + 7.0_f64 / 1536.0_f64 * t46918 - 7.0_f64 / 192.0_f64 * t46920 - 5.0_f64 / 256.0_f64 * t2643 * t9646 * t13242 * t9647 + 7.0_f64 / 1536.0_f64 * t46926 - t46929 + 7.0_f64 / 1536.0_f64 * t46930 - t4167 * t9663 / 3072.0_f64 - 15.0_f64 / 128.0_f64 * t2623 * t13186 + 7.0_f64 / 1536.0_f64 * t46936;
    t46938
}
