//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 209/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk209<F: Float>(t233: F, t251: F, t869: F, t689: F, t234: F, t786: F, t72: F, t686: F, t822: F, t837: F, t860: F, t213: F, t820: F) -> (F, F, F, F, F, F, F, F) {
    let t870 = t233 * t251;
    let t871 = t869 * t870;
    let t873 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t871;
    let t874 = t786 * t234;
    let t875 = t251 * t72;
    let t878 = F::cast_from(0.9757440539382783019e-2_f64) * t874 * t875 * t686;
    let t879 = t822 * t251;
    let t880 = t879 * t837;
    let t883 = t234 * t860;
    let t886 = -t873 + t878 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t880 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t883;
    (t870, t871, t873, t874, t875, t878, t879, t886)
}
