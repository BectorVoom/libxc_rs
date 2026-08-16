//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2268/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2268(t1512: f64, t41340: f64, t4236: f64, t9671: f64, t4166: f64, t9973: f64, t41354: f64, t13198: f64, t2697: f64, t13302: f64, t9638: f64, t13306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46951 = t41340 * t1512;
    let t46952 = 119.0_f64 / 4608.0_f64 * t46951;
    let t46953 = t9671 * t4236;
    let t46954 = 119.0_f64 / 4608.0_f64 * t46953;
    let t46957 = t4166 * t9973;
    let t46960 = t41354 * t1512;
    let t46962 = t2697 * t13198;
    let t46974 = t9638 * t13302;
    let t46980 = t9638 * t13306;
    (t46952, t46954, t46957, t46960, t46962, t46974, t46980)
}
