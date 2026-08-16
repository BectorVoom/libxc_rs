//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1164/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1164<F: Float>(t1358: F, t7242: F, t689: F, t2023: F, t786: F, t1364: F, t533: F, t7021: F, t816: F, t1941: F, t540: F) -> (F, F, F, F, F, F) {
    let t7243 = t7242 * t1358;
    let t7245 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t7243;
    let t7246 = t786 * t2023;
    let t7248 = F::cast_from(0.9757440539382783019e-2_f64) * t7246 * t1364;
    let t7250 = t7021 * t533 * t816;
    let t7251 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t7250;
    let t7252 = t1941 * t540;
    (t7243, t7245, t7246, t7248, t7251, t7252)
}
