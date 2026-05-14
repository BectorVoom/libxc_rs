//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 228/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk228<F: Float>(t177: F, t738: F, t687: F, t689: F, t693: F, t698: F) -> (F, F) {
    let t739 = t177 * t738;
    let t744 = -0.86308333333333333334e0 * t687 - 0.301925e0 * t689 - 0.5501625e-1 * t693 - 0.82785e-1 * t698;
    (t739, t744)
}
