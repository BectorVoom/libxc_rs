//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2361/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2361(t1294: f64, t39336: f64, t3691: f64, t9905: f64, t9892: f64, t2368: f64, t747: f64, t9711: f64, t9810: f64, t9844: f64, t39321: f64, t677: f64, t9713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39338 = 0.21053605041484726346e2_f64 * t1294 * t39336;
    let t39339 = t3691 * t9905;
    let t39341 = t3691 * t9892;
    let t39344 = t2368 * t9711 * t747;
    let t39346 = 0.46785788981077169656e1_f64 * t1294 * t39344;
    let t39347 = t9810 * t9844;
    let t39349 = 0.19263893255070628432e1_f64 * t39321 * t39347;
    let t39354 = t677 * t9713;
    (t39338, t39339, t39341, t39344, t39346, t39347, t39349, t39354)
}
