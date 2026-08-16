//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2502/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2502(t1512: f64, t41354: f64, t13198: f64, t2697: f64, t13302: f64, t9638: f64, t13306: f64, t13248: f64, t13258: f64, t1484: f64, t2631: f64, t4233: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46960 = t41354 * t1512;
    let t46962 = t2697 * t13198;
    let t46974 = t9638 * t13302;
    let t46980 = t9638 * t13306;
    let t46998 = t13258 * t13248;
    let t47012 = t1484 * t2631;
    let t47017 = t4233 * t828;
    (t46960, t46962, t46974, t46980, t46998, t47012, t47017)
}
