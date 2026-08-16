//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 973/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk973(t36119: f64, t46109: f64, t36103: f64, t46106: f64, t36110: f64, t36: f64, t5840: f64, t262: f64, t2115: f64, t41146: f64, t41160: f64, t41171: f64, t46084: f64, t46087: f64, t46090: f64, t46093: f64, t46096: f64, t46099: f64, t46102: f64, t46107: f64) -> (f64, f64, f64) {
    let t46110 = t36119 * t46109;
    let t46112 = t36103 * t46106;
    let t46114 = t36110 * t46109;
    let t46116 = t36 * t5840;
    let t46117 = t262 * t46116;
    let t46118 = t2115 * t46117;
    let t46120 = -0.2727466165424534173e-1_f64 * t46084 + 0.13637330827122670865e-1_f64 * t46087 - 0.2727466165424534173e-1_f64 * t46090 - 0.2727466165424534173e-1_f64 * t46093 + 0.68186654135613354324e-1_f64 * t46096 - 0.13637330827122670865e0_f64 * t46099 + 0.45457769423742236216e-1_f64 * t46102 - 0.15965655602485078086e0_f64 * t41146 + 0.7080615522698976714e-2_f64 * t41160 - t41171 - 0.5454932330849068346e-1_f64 * t46107 + 0.13637330827122670865e0_f64 * t46110 - 0.63504270469206447405e-2_f64 * t46112 + 0.10160683275073031585e-1_f64 * t46114 + 0.9072038638458063915e-4_f64 * t46118;
    (t46116, t46117, t46120)
}
