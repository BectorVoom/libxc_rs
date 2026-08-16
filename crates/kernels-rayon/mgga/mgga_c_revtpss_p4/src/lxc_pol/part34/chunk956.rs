//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 956/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk956(t22984: f64, t23058: f64, t1343: f64, t1450: f64, t198: f64, t22768: f64, t22791: f64, t22809: f64, t22919: f64, t22920: f64, t22921: f64, t22922: f64, t532: f64, t9394: f64, t9396: f64, t9409: f64, t9412: f64, t9415: f64, t9421: f64, t9427: f64) -> (f64, f64) {
    let t23059 = t22984 + t23058;
    let t23063 = t1450 * t198 * t23059 * t532 + 3.0_f64 * t1343 * t198 * t22809 - t22768 + t22791 + t22919 - t22920 + t22921 + t22922 + t9394 - t9396 + t9409 - t9412 - t9415 + t9421 - t9427;
    (t23059, t23063)
}
