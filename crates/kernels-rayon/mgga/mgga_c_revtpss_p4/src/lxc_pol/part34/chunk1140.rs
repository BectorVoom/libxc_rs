//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1140/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1140(t2013: f64, t28166: f64, t531: f64, t7933: f64, t8995: f64, t2033: f64, t9593: f64, t116: f64, t7741: f64, t27240: f64, t27246: f64, t27251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28167 = t2013 * t28166;
    let t28172 = t531 * t7933;
    let t28196 = t2013 * t8995;
    let t28197 = t2033 * t9593;
    let t28276 = t116 * t7741;
    let t28330 = 0.11433071498151929859e-3_f64 * t27240;
    let t28333 = 7.0_f64 / 72.0_f64 * t27246;
    let t28335 = 0.2032800112371413129e-3_f64 * t27251;
    (t28167, t28172, t28196, t28197, t28276, t28330, t28333, t28335)
}
