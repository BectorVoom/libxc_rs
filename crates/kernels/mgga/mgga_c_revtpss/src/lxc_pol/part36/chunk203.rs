//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 203/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk203<F: Float>(t169: F, t164: F, t687: F, t689: F, t693: F, t698: F, t172: F) -> (F, F, F, F, F) {
    let t722 = t169 * t169;
    let t723 = F::new(1.0) / t722;
    let t724 = t164 * t723;
    let t729 = -F::new(0.1176575e1) * t687 - F::new(0.516475e0) * t689 - F::new(0.2103875e0) * t693 - F::new(0.104195e0) * t698;
    let t730 = F::new(1.0) / t172;
    (t722, t723, t724, t729, t730)
}
