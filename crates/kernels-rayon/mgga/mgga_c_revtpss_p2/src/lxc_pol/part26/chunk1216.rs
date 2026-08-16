//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1216/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1216(t25898: f64, t7527: f64, t94849: f64, t25921: f64, t25930: f64, t26257: f64, t26304: f64, t26305: f64, t26347: f64, t27868: f64, t28911: f64, t4056: f64, t46422: f64, t543: f64, t7295: f64, t7301: f64, t7506: f64, t7511: f64, t94705: f64, t94737: f64, t94823: f64, t94825: f64, t96443: f64, t96473: f64, t96486: f64, t96491: f64, t96500: f64, t96503: f64, t9659: f64) -> f64 {
    let t96506 = t94849 * t25898 * t7527;
    let t96508 = 0.78062653693846795158e1_f64 * t94823 * t26304 * t94825 - 0.39512695097613069591e1_f64 * t7511 * t9659 - t96473 + 0.13010442282307799193e1_f64 * t25921 * t26257 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t7506 * t4056 * t543 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t96443 * t543 + 0.29272321618148349057e-1_f64 * t96486 - 0.52041769129231196772e1_f64 * t94705 * t26305 + t96491 + 0.52041769129231196772e1_f64 * t25930 * t28911 * t94737 - 0.26020884564615598386e1_f64 * t27868 * t28911 * t46422 + 0.52041769129231196772e1_f64 * t25921 * t26347 - 0.77108554593144223218e-1_f64 * t96500 - 0.72280234901709995519e-3_f64 * t96503 + 0.72280234901709995519e-3_f64 * t96506;
    t96508
}
