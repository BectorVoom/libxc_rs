//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1072/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1072<F: Float>(t2103: F, t47567: F, t1364: F, t26338: F, t786: F, t26261: F, t40270: F, t25950: F, t26271: F, t10073: F, t25920: F, t26260: F, t25898: F, t7527: F, t94849: F, t25921: F, t25930: F, t26257: F, t26304: F, t26305: F, t26347: F, t27868: F, t28911: F, t4056: F, t46422: F, t543: F, t7295: F, t7301: F, t7506: F, t7511: F, t94705: F, t94737: F, t94823: F, t94825: F, t96443: F, t9659: F) -> (F,) {
    let t96473 = 0.81814717454467823679e-4 * t47567 * t2103;
    let t96486 = t786 * t26338 * t1364;
    let t96491 = 0.96373646535613327356e-3 * t40270 * t26261;
    let t96500 = t25950 * t26271;
    let t96503 = t10073 * t25920 * t26260;
    let t96506 = t94849 * t25898 * t7527;
    let t96508 = 0.78062653693846795158e1 * t94823 * t26304 * t94825 - 0.39512695097613069591e1 * t7511 * t9659 - t96473 + 0.13010442282307799193e1 * t25921 * t26257 + 0.13010442282307799193e1 * t7295 * t7301 * t7506 * t4056 * t543 + 0.4336814094102599731e0 * t7295 * t7301 * t96443 * t543 + 0.29272321618148349057e-1 * t96486 - 0.52041769129231196772e1 * t94705 * t26305 + t96491 + 0.52041769129231196772e1 * t25930 * t28911 * t94737 - 0.26020884564615598386e1 * t27868 * t28911 * t46422 + 0.52041769129231196772e1 * t25921 * t26347 - 0.77108554593144223218e-1 * t96500 - 0.72280234901709995519e-3 * t96503 + 0.72280234901709995519e-3 * t96506;
    (t96508,)
}
