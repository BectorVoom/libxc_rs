//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 985/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk985(t3851: f64, t46261: f64, t36166: f64, t36168: f64, t43615: f64, t46232: f64, t46235: f64, t46238: f64, t46242: f64, t46244: f64, t46246: f64, t46248: f64, t46250: f64, t46252: f64, t46254: f64, t46256: f64, t46259: f64) -> f64 {
    let t46262 = t3851 * t46261;
    let t46264 = 0.5987120850931904282e-1_f64 * t46232 - 0.39828462315181744017e-2_f64 * t46235 + 0.79656924630363488034e-2_f64 * t46238 - t43615 - 0.97567895348519921636e-1_f64 * t36166 + 0.14635184302277988245e0_f64 * t36168 + 0.39828462315181744016e-2_f64 * t46242 - 0.13939961810313610406e-1_f64 * t46244 + 0.22303938896501776649e-1_f64 * t46246 + 0.2993560425465952141e0_f64 * t46248 - 0.11974241701863808564e0_f64 * t46250 + 0.5987120850931904282e-1_f64 * t46252 + 0.5987120850931904282e-1_f64 * t46254 - 0.11974241701863808564e0_f64 * t46256 - 0.5987120850931904282e-1_f64 * t46259 + 0.2993560425465952141e-1_f64 * t46262;
    t46264
}
