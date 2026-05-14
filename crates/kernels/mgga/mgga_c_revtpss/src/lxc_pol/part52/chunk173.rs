//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 173/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk173<F: Float>(t45: F, t57: F, t190: F, t606: F, t706: F, t78: F, t81: F, t150: F, t169: F, t164: F, t687: F, t689: F, t693: F, t698: F, t172: F, t182: F, t177: F, t185: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t707 = t190 * t606;
    let t709 = 4.0 * t706 * t707;
    let t712 = piecewise3(t151, 0.0, 4.0 / 3.0 * t78 * t606);
    let t715 = piecewise3(t155, 0.0, -4.0 / 3.0 * t81 * t606);
    let t716 = t712 + t715;
    let t717 = t150 * t716;
    let t718 = t717 * t190;
    let t722 = t169 * t169;
    let t723 = 1.0 / t722;
    let t724 = t164 * t723;
    let t729 = -0.1176575e1 * t687 - 0.516475e0 * t689 - 0.2103875e0 * t693 - 0.104195e0 * t698;
    let t730 = 1.0 / t172;
    let t731 = t729 * t730;
    let t737 = t182 * t182;
    let t738 = 1.0 / t737;
    let t739 = t177 * t738;
    let t744 = -0.86308333333333333334e0 * t687 - 0.301925e0 * t689 - 0.5501625e-1 * t693 - 0.82785e-1 * t698;
    let t745 = 1.0 / t185;
    (t707, t709, t716, t717, t718, t722, t723, t724, t729, t730, t731, t737, t738, t739, t744, t745)
}
