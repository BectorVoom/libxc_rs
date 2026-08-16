//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2473/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2473(t1020: f64, t21595: f64, t248: f64, t3101: f64, t14511: f64, t17655: f64, t10883: f64, t21403: f64, t1041: f64, t21130: f64, t42592: f64, t21594: f64, t376: f64) -> (f64, f64, f64, f64, f64) {
    let t70346 = t1020 * t248 * t3101 * t21595;
    let t70351 = t14511 * t17655;
    let t70363 = t10883 * t248 * t3101 * t21403;
    let t70389 = t1041 * t248 * t42592 * t21130;
    let t70391 = t376 * t21594;
    (t70346, t70351, t70363, t70389, t70391)
}
