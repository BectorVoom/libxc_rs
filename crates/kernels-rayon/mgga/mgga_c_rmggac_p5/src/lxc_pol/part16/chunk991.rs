//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 991/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk991(t27091: f64, t45572: f64, t25877: f64, t45577: f64, t45730: f64, t5271: f64, t46357: f64, t5259: f64, t40823: f64, t9708: f64, t45726: f64, t46529: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46660 = t27091 * t45572;
    let t46662 = t25877 * t45577;
    let t46664 = t5271 * t45730;
    let t46669 = t5259 * t46357;
    let t46671 = t40823 * t9708;
    let t46673 = t5271 * t45726;
    let t46675 = t4669 * t46529;
    (t46660, t46662, t46664, t46669, t46671, t46673, t46675)
}
