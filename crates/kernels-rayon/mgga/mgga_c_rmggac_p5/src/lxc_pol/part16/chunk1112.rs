//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1112/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1112(t41257: f64, t41265: f64, t41271: f64, t41299: f64, t41302: f64, t43588: f64, t43592: f64, t43594: f64, t43596: f64, t46189: f64, t46191: f64, t46193: f64, t46195: f64, t46197: f64, t46199: f64, t46201: f64) -> f64 {
    let t49095 = -0.4838420607177634088e-3_f64 * t46189 + 0.56448240417072397693e-3_f64 * t46191 - 0.36366215538993788973e-1_f64 * t46193 - 0.12122071846331262991e0_f64 * t46195 + 0.58540737209111952978e0_f64 * t41257 - 0.12981128458281457309e-1_f64 * t41265 - 0.66380770525302906695e-3_f64 * t46197 + 0.2993560425465952141e-1_f64 * t46199 + 0.53218852008283593619e-1_f64 * t46201 - 0.41395376305853091643e-2_f64 * t41271 - t43588 - 0.90317184667315836312e-2_f64 * t41299 - 0.72732431077987577944e-1_f64 * t41302 + t43592 + t43594 - t43596;
    t49095
}
