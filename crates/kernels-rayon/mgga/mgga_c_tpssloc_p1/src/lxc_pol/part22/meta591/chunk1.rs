//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2107/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2107(t4166: f64, t9666: f64, t2693: f64, t4163: f64, t41008: f64, t4155: f64, t41115: f64, t4240: f64, t1512: f64, t41340: f64, t4236: f64, t9671: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46881 = t4166 * t9666;
    let t46886 = t4163 * t2693;
    let t46887 = 119.0_f64 / 4608.0_f64 * t46886;
    let t46911 = t41008 * t4155;
    let t46912 = 35.0_f64 / 24.0_f64 * t46911;
    let t46928 = t41115 * t4240;
    let t46929 = 119.0_f64 / 4608.0_f64 * t46928;
    let t46951 = t41340 * t1512;
    let t46952 = 119.0_f64 / 4608.0_f64 * t46951;
    let t46953 = t9671 * t4236;
    (t46881, t46887, t46912, t46929, t46952, t46953)
}
