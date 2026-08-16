//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2489/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2489(t4119: f64, t828: f64, t46528: f64, t842: f64, t4261: f64, t9601: f64, t1516: f64, t40965: f64, t13347: f64, t2697: f64, t13210: f64, t9638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46565 = t4119 * t828;
    let t46570 = t46528 * t842;
    let t46573 = t9601 * t4261;
    let t46577 = t40965 * t1516;
    let t46587 = t2697 * t13347;
    let t46595 = t9638 * t13210;
    (t46565, t46570, t46573, t46577, t46587, t46595)
}
