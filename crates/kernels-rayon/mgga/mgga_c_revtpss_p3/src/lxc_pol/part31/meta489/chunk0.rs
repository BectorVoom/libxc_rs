//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1788/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1788(t342: f64, t7135: f64, t1071: f64, t3140: f64, t1078: f64, t1982: f64, t1976: f64, t3057: f64, t989: f64, t11239: f64, t378: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25634 = t342 * t7135;
    let t25638 = t1071 * t3140;
    let t25640 = t1982 * t25638 * t1078;
    let t25651 = t3057 * t1976;
    let t25658 = t989 * t1976;
    let t25669 = t378 * t11239;
    let t25671 = t1982 * t25669 * t1078;
    let t25672 = t3143 * t1976;
    (t25634, t25640, t25651, t25658, t25671, t25672)
}
