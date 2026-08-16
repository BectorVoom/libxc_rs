//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1026/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1026<F: Float>(t12247: F, t408: F, t12228: F, t3435: F, t3418: F, t698: F, t240: F, t3698: F, t3361: F, t635: F, t10356: F, t141: F) -> (F, F, F, F, F) {
    let t12248 = t408 * t12247;
    let t12249 = t12228 * t3435;
    let t12251 = F::cast_from(0.96491876992155210402e2_f64) * t12248 * t12249;
    let t12252 = t698 * t3418;
    let t12254 = t240 * t3698;
    let t12256 = F::cast_from(1.0_f64) / t3361 / t635;
    let t12257 = t12256 * t10356;
    let t12258 = t12254 * t12257;
    let t12259 = t141 * t12258;
    (t12251, t12252, t12256, t12257, t12259)
}
