//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2481/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2481(t39: f64, t9287: f64, t51: f64, t9300: f64, t12566: f64, t604: f64, t2239: f64, t3951: f64, t4199: f64, t9919: f64, t12887: f64, t67: f64, t758: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45970 = t39 * t9287;
    let t45974 = t51 * t9300;
    let t46099 = t12566 * t604;
    let t46104 = t3951 * t2239;
    let t46125 = t4199 * t9919;
    let t46128 = t12887 * t67 * t758;
    (t45970, t45974, t46099, t46104, t46125, t46128)
}
