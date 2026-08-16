//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 988/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk988(t262: f64, t46541: f64, t35929: f64, t46261: f64, t5271: f64, t46437: f64, t5259: f64, t45166: f64, t5148: f64, t44732: f64, t46419: f64, t46423: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46542 = t262 * t46541;
    let t46543 = t35929 * t46542;
    let t46554 = t5271 * t46261;
    let t46556 = t5259 * t46437;
    let t46599 = t5148 * t45166;
    let t46603 = t5271 * t44732;
    let t46605 = t5259 * t46419;
    let t46607 = t4669 * t46423;
    (t46542, t46543, t46554, t46556, t46599, t46603, t46605, t46607)
}
