//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2086/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2086(t4797: f64, t7131: f64, t15682: f64, t25517: f64, t4857: f64, t16163: f64, t7122: f64, t15772: f64, t7132: f64, t15984: f64, t1058: f64, t27464: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100230 = t4797 * t7131;
    let t100240 = 0.3811023832717309953e-3_f64 * t25517 * t15682;
    let t100255 = t4857 * t7131;
    let t100261 = 0.57165357490759649296e-3_f64 * t7122 * t16163;
    let t100262 = t7132 * t15772;
    let t100268 = t25517 * t15984;
    let t100270 = t27464 * t1058;
    (t100230, t100240, t100255, t100261, t100262, t100268, t100270)
}
