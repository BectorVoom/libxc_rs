//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 720/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk720<F: Float>(t1989: F, t3336: F, t2033: F, t531: F, t2022: F, t212: F, t1358: F, t689: F, t2023: F, t786: F, t1364: F, t533: F, t7021: F, t816: F) -> (F, F, F, F, F, F, F, F) {
    let t7181 = t1989 * t3336;
    let t7237 = t531 * t2033;
    let t7242 = t212 * t2022;
    let t7243 = t7242 * t1358;
    let t7245 = F::new(0.54878743191129263322e-2) * t689 * t7243;
    let t7246 = t786 * t2023;
    let t7248 = F::new(0.9757440539382783019e-2) * t7246 * t1364;
    let t7250 = t7021 * t533 * t816;
    (t7181, t7237, t7242, t7243, t7245, t7246, t7248, t7250)
}
