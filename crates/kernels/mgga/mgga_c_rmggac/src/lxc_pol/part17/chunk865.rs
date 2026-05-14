//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 865/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk865<F: Float>(t3851: F, t45730: F, t25640: F, t45568: F, t333: F, t9876: F, t3814: F, t9872: F, t36166: F, t36168: F, t43615: F, t46232: F, t46235: F, t46238: F, t46242: F, t46244: F, t46246: F, t46248: F, t46250: F, t46252: F) -> (F, F, F) {
    let t46254 = t3851 * t45730;
    let t46256 = t25640 * t45568;
    let t46258 = t9876 * t333;
    let t46259 = t3814 * t46258;
    let t46261 = t9872 * t333;
    let t46262 = t3851 * t46261;
    let t46264 = 0.5987120850931904282e-1 * t46232 - 0.39828462315181744017e-2 * t46235 + 0.79656924630363488034e-2 * t46238 - t43615 - 0.97567895348519921636e-1 * t36166 + 0.14635184302277988245e0 * t36168 + 0.39828462315181744016e-2 * t46242 - 0.13939961810313610406e-1 * t46244 + 0.22303938896501776649e-1 * t46246 + 0.2993560425465952141e0 * t46248 - 0.11974241701863808564e0 * t46250 + 0.5987120850931904282e-1 * t46252 + 0.5987120850931904282e-1 * t46254 - 0.11974241701863808564e0 * t46256 - 0.5987120850931904282e-1 * t46259 + 0.2993560425465952141e-1 * t46262;
    (t46258, t46261, t46264)
}
