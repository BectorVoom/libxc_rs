//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 243/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk243(t177: f64, t738: f64, t687: f64, t689: f64, t693: f64, t698: f64) -> (f64, f64) {
    let t739 = t177 * t738;
    let t744 = -0.86308333333333333334e0_f64 * t687 - 0.301925e0_f64 * t689 - 0.5501625e-1_f64 * t693 - 0.82785e-1_f64 * t698;
    (t739, t744)
}
