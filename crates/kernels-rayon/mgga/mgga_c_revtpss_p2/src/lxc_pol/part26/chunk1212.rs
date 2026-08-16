//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1212/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1212(t2453: f64, t3908: f64, t7507: f64, t3923: f64, t7506: f64, t2435: f64, t26301: f64, t7289: f64, t96276: f64, t25924: f64, t25930: f64, t25933: f64, t26304: f64, t26335: f64, t27868: f64, t4077: f64, t46433: f64, t543: f64, t7292: f64, t7295: f64, t7301: f64, t7511: f64, t94752: f64, t96378: f64, t96380: f64, t96382: f64, t96392: f64, t96398: f64, t96401: f64, t9652: f64) -> (f64, f64) {
    let t96403 = t2453 * t7507 * t3908;
    let t96405 = t7506 * t3923;
    let t96410 = t2435 * t26301;
    let t96412 = t7289 * t96276;
    let t96420 = -0.23132566377943266966e0_f64 * t96378 + 0.51405703062096148814e-2_f64 * t96380 + 0.51405703062096148814e-2_f64 * t96382 - 0.13010442282307799193e1_f64 * t7292 * t26335 + 0.39512695097613069591e1_f64 * t7511 * t9652 - 0.78062653693846795158e1_f64 * t7295 * t25924 * t7506 * t4077 - 0.52041769129231196772e1_f64 * t25930 * t96392 * t25933 - 0.72280234901709995519e-3_f64 * t96398 + t96401 + 0.34697458558045176417e-2_f64 * t96403 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t96405 * t543 - 0.21951497276451705329e-1_f64 * t96410 + 0.51405703062096148812e-1_f64 * t96412 - 0.26020884564615598386e1_f64 * t25930 * t26304 * t94752 + 0.13010442282307799193e1_f64 * t27868 * t26304 * t46433;
    (t96405, t96420)
}
