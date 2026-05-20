//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 177/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk177<F: Float>(t172: F, t729: F, t182: F, t177: F, t687: F, t689: F, t693: F, t698: F, t185: F) -> (F, F, F, F, F, F, F) {
    let t730 = F::new(1.0) / t172;
    let t731 = t729 * t730;
    let t737 = t182 * t182;
    let t738 = F::new(1.0) / t737;
    let t739 = t177 * t738;
    let t744 = -F::cast_from(0.86308333333333333334e0_f64) * t687 - F::new(0.301925e0) * t689 - F::new(0.5501625e-1) * t693 - F::new(0.82785e-1) * t698;
    let t745 = F::new(1.0) / t185;
    (t730, t731, t737, t738, t739, t744, t745)
}
