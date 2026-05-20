//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 761/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk761<F: Float>(t1364: F, t7246: F, t533: F, t7021: F, t816: F, t1941: F, t540: F, t1372: F, t546: F, t550: F, t7028: F, t807: F) -> (F, F, F, F, F, F) {
    let t7248 = F::cast_from(0.9757440539382783019e-2_f64) * t7246 * t1364;
    let t7250 = t7021 * t533 * t816;
    let t7251 = F::new(7.0) / F::new(288.0) * t7250;
    let t7252 = t1941 * t540;
    let t7253 = t7252 * t1372;
    let t7256 = t546 * t7028 * t550;
    let t7257 = t807 * t7256;
    (t7248, t7251, t7252, t7253, t7256, t7257)
}
