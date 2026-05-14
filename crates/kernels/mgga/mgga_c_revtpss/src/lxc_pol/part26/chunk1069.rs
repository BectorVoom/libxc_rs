//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1069/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1069<F: Float>(t3923: F, t7506: F, t2435: F, t26301: F, t7289: F, t96276: F, t25924: F, t25930: F, t25933: F, t26304: F, t26335: F, t27868: F, t4077: F, t46433: F, t543: F, t7292: F, t7295: F, t7301: F, t7511: F, t94752: F, t96378: F, t96380: F, t96382: F, t96392: F, t96398: F, t96401: F, t96403: F, t9652: F) -> (F, F) {
    let t96405 = t7506 * t3923;
    let t96410 = t2435 * t26301;
    let t96412 = t7289 * t96276;
    let t96420 = -0.23132566377943266966e0 * t96378 + 0.51405703062096148814e-2 * t96380 + 0.51405703062096148814e-2 * t96382 - 0.13010442282307799193e1 * t7292 * t26335 + 0.39512695097613069591e1 * t7511 * t9652 - 0.78062653693846795158e1 * t7295 * t25924 * t7506 * t4077 - 0.52041769129231196772e1 * t25930 * t96392 * t25933 - 0.72280234901709995519e-3 * t96398 + t96401 + 0.34697458558045176417e-2 * t96403 + 0.13010442282307799193e1 * t7295 * t7301 * t96405 * t543 - 0.21951497276451705329e-1 * t96410 + 0.51405703062096148812e-1 * t96412 - 0.26020884564615598386e1 * t25930 * t26304 * t94752 + 0.13010442282307799193e1 * t27868 * t26304 * t46433;
    (t96405, t96420)
}
