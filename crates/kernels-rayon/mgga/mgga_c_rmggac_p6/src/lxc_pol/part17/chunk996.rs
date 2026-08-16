//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 996/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk996(t118: f64, t25854: f64, t27048: f64, t27101: f64, t326: f64, t46375: f64, t46379: f64, t46382: f64, t46386: f64, t46389: f64, t46392: f64, t46395: f64, t46398: f64, t46400: f64, t46403: f64, t46406: f64, t46409: f64) -> f64 {
    let t46411 = -0.11974241701863808564e0_f64 * t326 * t46375 - 0.39914139006212695214e-1_f64 * t118 * t46379 - 0.59871208509319042821e-1_f64 * t326 * t46382 + 0.17961362552795712846e0_f64 * t46386 + 0.11974241701863808564e0_f64 * t46389 + 0.71845450211182851384e0_f64 * t46392 + 0.17961362552795712846e0_f64 * t46395 - 0.17961362552795712846e0_f64 * t46398 - 0.47896966807455234256e0_f64 * t27101 * t46400 + 0.71845450211182851384e0_f64 * t25854 * t46403 + 0.71845450211182851384e0_f64 * t27048 * t46406 + 0.40911992481368012592e-1_f64 * t46409;
    t46411
}
