//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1111/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1111(t12083: f64, t67: f64, t758: f64, t2505: f64, t2527: f64, t1294: f64, t3691: f64, t9905: f64, t9892: f64, t2368: f64, t747: f64, t9711: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39334 = t12083 * t67 * t758;
    let t39335 = 0.73245789224026180216e-3_f64 * t39334;
    let t39336 = t2527 * t2505;
    let t39338 = 0.21053605041484726346e2_f64 * t1294 * t39336;
    let t39339 = t3691 * t9905;
    let t39340 = 0.14035736694323150897e2_f64 * t39339;
    let t39341 = t3691 * t9892;
    let t39342 = 0.20779030926817756511e3_f64 * t39341;
    let t39344 = t2368 * t9711 * t747;
    (t39335, t39336, t39338, t39340, t39342, t39344)
}
