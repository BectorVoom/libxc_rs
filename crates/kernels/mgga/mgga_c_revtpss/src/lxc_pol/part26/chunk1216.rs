//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1216/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1216<F: Float>(t25898: F, t7527: F, t94849: F, t25921: F, t25930: F, t26257: F, t26304: F, t26305: F, t26347: F, t27868: F, t28911: F, t4056: F, t46422: F, t543: F, t7295: F, t7301: F, t7506: F, t7511: F, t94705: F, t94737: F, t94823: F, t94825: F, t96443: F, t96473: F, t96486: F, t96491: F, t96500: F, t96503: F, t9659: F) -> F {
    let t96506 = t94849 * t25898 * t7527;
    let t96508 = F::cast_from(0.78062653693846795158e1_f64) * t94823 * t26304 * t94825 - F::cast_from(0.39512695097613069591e1_f64) * t7511 * t9659 - t96473 + F::cast_from(0.13010442282307799193e1_f64) * t25921 * t26257 + F::cast_from(0.13010442282307799193e1_f64) * t7295 * t7301 * t7506 * t4056 * t543 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t96443 * t543 + F::cast_from(0.29272321618148349057e-1_f64) * t96486 - F::cast_from(0.52041769129231196772e1_f64) * t94705 * t26305 + t96491 + F::cast_from(0.52041769129231196772e1_f64) * t25930 * t28911 * t94737 - F::cast_from(0.26020884564615598386e1_f64) * t27868 * t28911 * t46422 + F::cast_from(0.52041769129231196772e1_f64) * t25921 * t26347 - F::cast_from(0.77108554593144223218e-1_f64) * t96500 - F::cast_from(0.72280234901709995519e-3_f64) * t96503 + F::cast_from(0.72280234901709995519e-3_f64) * t96506;
    t96508
}
