//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1081/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1081(t482: f64, t5245: f64, t371: f64, t372: f64, t1234: f64, t1803: f64, t225: f64, t5219: f64, t480: f64, t3623: f64, t4890: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5318 = t482 * t5245;
    let t5320 = t371 * t372 * t5318;
    let t5323 = t1234 * t1803;
    let t5326 = t5219 * t225;
    let t5327 = t5326 * t480;
    let t5330 = t3623 * t4890;
    (t5318, t5320, t5323, t5326, t5327, t5330)
}
