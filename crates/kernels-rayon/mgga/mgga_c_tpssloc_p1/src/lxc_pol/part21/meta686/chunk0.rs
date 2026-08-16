//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2501/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2501(t13316: f64, t9638: f64, t41115: f64, t4240: f64, t13278: f64, t2686: f64, t13173: f64, t2639: f64, t1512: f64, t41340: f64, t4236: f64, t9671: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46926 = t9638 * t13316;
    let t46928 = t41115 * t4240;
    let t46930 = t13278 * t2686;
    let t46936 = t2639 * t13173;
    let t46951 = t41340 * t1512;
    let t46953 = t9671 * t4236;
    (t46926, t46928, t46930, t46936, t46951, t46953)
}
