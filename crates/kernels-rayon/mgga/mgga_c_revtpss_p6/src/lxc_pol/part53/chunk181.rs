//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 181/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk181(t172: f64, t729: f64, t182: f64, t177: f64, t687: f64, t689: f64, t693: f64, t698: f64, t185: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t730 = 1.0_f64 / t172;
    let t731 = t729 * t730;
    let t737 = t182 * t182;
    let t738 = 1.0_f64 / t737;
    let t739 = t177 * t738;
    let t744 = -0.86308333333333333334e0_f64 * t687 - 0.301925e0_f64 * t689 - 0.5501625e-1_f64 * t693 - 0.82785e-1_f64 * t698;
    let t745 = 1.0_f64 / t185;
    (t730, t731, t737, t738, t739, t744, t745)
}
