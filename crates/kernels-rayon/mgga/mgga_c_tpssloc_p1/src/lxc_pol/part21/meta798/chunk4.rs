//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2776/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2776(t1512: f64, t46667: f64, t16903: f64, t9638: f64, t41008: f64, t5568: f64, t13225: f64, t13251: f64, t13262: f64, t16872: f64, t2686: f64, t41084: f64, t41086: f64, t46692: f64, t46876: f64, t46882: f64, t46884: f64, t46886: f64, t46911: f64, t46918: f64, t46920: f64, t46926: f64, t46928: f64, t47017: f64, t47285: f64) -> f64 {
    let t58731 = t46667 * t1512;
    let t58735 = t9638 * t16903;
    let t58744 = t41008 * t5568;
    let t58754 = -t16872 * t2686 / 3072.0_f64 + 7.0_f64 / 1152.0_f64 * t58731 + 595.0_f64 / 5184.0_f64 * t46876 - 7.0_f64 / 1152.0_f64 * t46882 - 7.0_f64 / 576.0_f64 * t58735 - 7.0_f64 / 2304.0_f64 * t46884 + 119.0_f64 / 3456.0_f64 * t46886 - t13262 * t46692 * t47285 * t47017 / 128.0_f64 + 35.0_f64 / 18.0_f64 * t46911 + 35.0_f64 / 72.0_f64 * t58744 + 455.0_f64 / 324.0_f64 * t41084 - 35.0_f64 / 216.0_f64 * t41086 + t13251 * t13225 / 192.0_f64 + 7.0_f64 / 2304.0_f64 * t46918 - 7.0_f64 / 288.0_f64 * t46920 + 7.0_f64 / 2304.0_f64 * t46926 - 119.0_f64 / 3456.0_f64 * t46928;
    t58754
}
