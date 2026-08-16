//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 968/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk968(t46086: f64, t8750: f64, t46089: f64, t7603: f64, t46092: f64, t36110: f64, t46095: f64, t41329: f64, t46098: f64, t46101: f64, t8761: f64, t6415: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46152 = t8750 * t46086;
    let t46154 = t7603 * t46089;
    let t46156 = t7603 * t46092;
    let t46158 = t36110 * t46095;
    let t46160 = t41329 * t46098;
    let t46162 = t8761 * t46101;
    let t46164 = t649 * t6415;
    (t46152, t46154, t46156, t46158, t46160, t46162, t46164)
}
