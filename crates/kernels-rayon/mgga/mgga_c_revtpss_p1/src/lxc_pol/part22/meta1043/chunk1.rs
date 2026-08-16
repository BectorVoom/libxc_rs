//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3647/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3647(t12571: f64, t6552: f64, t43995: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64) -> (f64, f64) {
    let t68971 = 0.5848223622634646207e0_f64 * t12571 * t6552;
    let t68983 = 0.37083333333333333334e-1_f64 * t68253 + 0.41203703703703703704e-2_f64 * t68255 - 0.27469135802469135803e-2_f64 * t68257 + t43995 - 0.68672839506172839506e-2_f64 * t68262 + 0.10300925925925925926e-1_f64 * t68267 + 0.2225e0_f64 * t68271 + 0.37083333333333333334e-1_f64 * t68275 - 0.12361111111111111111e-1_f64 * t68277 - 0.12361111111111111111e-1_f64 * t68282 - 0.61805555555555555555e-2_f64 * t68287 - 0.37083333333333333333e-1_f64 * t68292;
    (t68971, t68983)
}
