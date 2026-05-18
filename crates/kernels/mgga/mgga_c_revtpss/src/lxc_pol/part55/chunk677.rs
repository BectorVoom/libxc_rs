//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 677/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk677<F: Float>(t7237: F, t7238: F, t2014: F, t2022: F, t212: F, t1358: F, t689: F, t2023: F, t786: F, t1364: F, t533: F, t7021: F, t816: F) -> (F, F, F, F, F, F, F, F) {
    let t7239 = t7237 * t7238;
    let t7241 = F::new(3.0) * t2014 * t7239;
    let t7242 = t212 * t2022;
    let t7243 = t7242 * t1358;
    let t7245 = F::new(0.54878743191129263322e-2) * t689 * t7243;
    let t7246 = t786 * t2023;
    let t7248 = F::new(0.9757440539382783019e-2) * t7246 * t1364;
    let t7250 = t7021 * t533 * t816;
    (t7239, t7241, t7242, t7243, t7245, t7246, t7248, t7250)
}
