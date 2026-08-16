//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 816/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk816(t8306: f64, t8402: f64, t2226: f64, t8397: f64, t2230: f64, t8998: f64, t2217: f64, t556: f64, t2147: f64, t1658: f64, t633: f64, t159: f64, t619: f64, t9367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9402 = t8306 * t8402;
    let t9407 = t8397 * t2226;
    let t9409 = t8998 * t2230;
    let t9413 = t2217 * t556;
    let t9414 = t2147 * t9413;
    let t9417 = t633 * t1658;
    let t9418 = t2147 * t9417;
    let t9422 = t619 * t159 * t9367;
    (t9402, t9407, t9409, t9413, t9414, t9417, t9418, t9422)
}
