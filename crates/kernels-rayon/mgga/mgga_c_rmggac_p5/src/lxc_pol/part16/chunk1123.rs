//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1123/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1123(t25854: f64, t27048: f64, t326: f64, t40806: f64, t44029: f64, t44035: f64, t46056: f64, t46059: f64, t46062: f64, t46064: f64, t46066: f64, t46069: f64, t48271: f64, t48278: f64, t48976: f64) -> f64 {
    let t49294 = -0.59871208509319042821e-1_f64 * t326 * t48976 - t44029 - 0.47896966807455234255e0_f64 * t40806 - 0.95793933614910468512e0_f64 * t46056 - 0.15965655602485078085e0_f64 * t46059 + t44035 + 0.71845450211182851384e0_f64 * t25854 * t48271 + 0.71845450211182851384e0_f64 * t27048 * t48278 - 0.40911992481368012596e-1_f64 * t46062 - 0.5987120850931904282e-1_f64 * t46064 + 0.11974241701863808564e0_f64 * t46066 - 0.17961362552795712846e0_f64 * t46069;
    t49294
}
