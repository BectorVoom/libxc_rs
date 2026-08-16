//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1114/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1114(t36168: f64, t41355: f64, t41358: f64, t46232: f64, t46235: f64, t46238: f64, t46242: f64, t46244: f64, t46246: f64, t46248: f64, t46250: f64, t46252: f64, t46254: f64, t46256: f64, t46259: f64, t46262: f64) -> f64 {
    let t49126 = 0.11974241701863808564e0_f64 * t46232 - 0.79656924630363488034e-2_f64 * t46235 + 0.15931384926072697607e-1_f64 * t46238 - 0.17779038707952519053e0_f64 * t41355 - t41358 + 0.2927036860455597649e0_f64 * t36168 + 0.79656924630363488034e-2_f64 * t46242 - 0.27879923620627220812e-1_f64 * t46244 + 0.44607877793003553299e-1_f64 * t46246 + 0.5987120850931904282e0_f64 * t46248 - 0.23948483403727617128e0_f64 * t46250 + 0.11974241701863808564e0_f64 * t46252 + 0.11974241701863808564e0_f64 * t46254 - 0.23948483403727617128e0_f64 * t46256 - 0.11974241701863808564e0_f64 * t46259 + 0.59871208509319042821e-1_f64 * t46262;
    t49126
}
