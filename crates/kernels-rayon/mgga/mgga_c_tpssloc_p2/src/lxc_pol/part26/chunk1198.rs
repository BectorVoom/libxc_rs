//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1198/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1198(t2553: f64, t868: f64, t2379: f64, t2749: f64, t776: f64, t2745: f64, t2678: f64, t829: f64, t828: f64, t9632: f64, t2631: f64, t1254: f64, t3633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46252 = t2553 * t868;
    let t46298 = t2379 * t868;
    let t46320 = t776 * t2749;
    let t46362 = t2745 * t868;
    let t46511 = t829 * t2678;
    let t46519 = t9632 * t828;
    let t46606 = t2553 * t828;
    let t47072 = t2379 * t828;
    let t47320 = t2631 * t776;
    let t51906 = t1254 * t3633;
    (t46252, t46298, t46320, t46362, t46511, t46519, t46606, t47072, t47320, t51906)
}
