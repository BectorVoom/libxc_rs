//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 982/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk982(t262: f64, t46427: f64, t7835: f64, t46278: f64, t7844: f64, t46261: f64, t7785: f64, t352: f64, t9872: f64, t7788: f64, t2350: f64, t5144: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46428 = t262 * t46427;
    let t46429 = t7835 * t46428;
    let t46431 = t262 * t46278;
    let t46432 = t7844 * t46431;
    let t46434 = t262 * t46261;
    let t46435 = t7785 * t46434;
    let t46437 = t9872 * t352;
    let t46438 = t262 * t46437;
    let t46439 = t7788 * t46438;
    let t46441 = t2350 * t5144;
    (t46428, t46429, t46431, t46432, t46434, t46435, t46437, t46438, t46439, t46441)
}
