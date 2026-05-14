//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1084/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1084<F: Float>(t25944: F, t94407: F, t2482: F, t7262: F, t814: F, t820: F, t844: F, t596: F, t7269: F, t25981: F, t843: F, t2681: F, t533: F, t816: F, t92993: F, t7259: F, t9709: F) -> (F, F, F, F, F, F, F, F) {
    let t94409 = 0.68540937416128198417e-2 * t25944 * t94407;
    let t94423 = t2482 * t7262 * t814;
    let t94429 = t820 * t7262 * t844;
    let t94443 = t2482 * t7269 * t596;
    let t94455 = t820 * t25981 * t843;
    let t94459 = t820 * t7262 * t2681;
    let t94471 = t92993 * t533 * t816;
    let t94472 = 455.0 / 1296.0 * t94471;
    let t94473 = t7259 * t9709;
    (t94409, t94423, t94429, t94443, t94455, t94459, t94472, t94473)
}
