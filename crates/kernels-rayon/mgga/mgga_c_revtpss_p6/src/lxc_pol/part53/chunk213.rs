//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 213/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk213(t233: f64, t251: f64, t869: f64, t689: f64, t234: f64, t786: f64, t72: f64, t686: f64, t822: f64, t837: f64, t860: f64, t213: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t870 = t233 * t251;
    let t871 = t869 * t870;
    let t873 = 0.54878743191129263322e-2_f64 * t689 * t871;
    let t874 = t786 * t234;
    let t875 = t251 * t72;
    let t878 = 0.9757440539382783019e-2_f64 * t874 * t875 * t686;
    let t879 = t822 * t251;
    let t880 = t879 * t837;
    let t883 = t234 * t860;
    let t886 = -t873 + t878 - 0.65854491829355115987e0_f64 * t820 * t880 + 0.65854491829355115987e0_f64 * t213 * t883;
    (t870, t871, t873, t874, t875, t878, t879, t886)
}
