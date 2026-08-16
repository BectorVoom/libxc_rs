//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1079/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1079(t5466: f64, t639: f64, t237: f64, t5762: f64, t1847: f64, t1854: f64, t5775: f64, t659: f64, t11817: f64, t204: f64, t208: f64) -> (f64, f64, f64, f64, f64) {
    let t17280 = t5466 * t639;
    let t17312 = t237 * t5762;
    let t17326 = t1847 * t1854;
    let t17329 = t659 * t5775;
    let t17348 = t204 * t11817 * t208;
    (t17280, t17312, t17326, t17329, t17348)
}
