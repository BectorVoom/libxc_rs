//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2500/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2500(t4166: f64, t9666: f64, t2635: f64, t13337: f64, t838: f64, t2693: f64, t4163: f64, t41008: f64, t4155: f64, t13076: f64, t9638: f64, t13322: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46881 = t4166 * t9666;
    let t46882 = t46881 * t2635;
    let t46884 = t13337 * t838;
    let t46886 = t4163 * t2693;
    let t46911 = t41008 * t4155;
    let t46918 = t9638 * t13076;
    let t46920 = t9638 * t13322;
    (t46882, t46884, t46886, t46911, t46918, t46920)
}
