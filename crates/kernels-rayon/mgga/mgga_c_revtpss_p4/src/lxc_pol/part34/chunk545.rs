//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 545/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk545(t1012: f64, t1224: f64, t3698: f64, t1234: f64, t1803: f64, t225: f64, t5219: f64) -> (f64, f64, f64, f64) {
    let t5308 = t1012 * t1224;
    let t5312 = t1012 * t3698;
    let t5323 = t1234 * t1803;
    let t5326 = t5219 * t225;
    (t5308, t5312, t5323, t5326)
}
