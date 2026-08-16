//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 774/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk774(t1317: f64, t3853: f64, t1333: f64, t3863: f64, t27: f64, t583: f64, t521: f64, t19: f64, t596: f64, t182: f64, t2490: f64, t2495: f64, t9368: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9395 = t1317 * t3853;
    let t9396 = 12.0_f64 * t9395;
    let t9408 = t3863 * t1333;
    let t9409 = 96.0_f64 * t9408;
    let t9410 = t583 * t27;
    let t9411 = t9410 * t521;
    let t9412 = 240.0_f64 * t9411;
    let t9413 = t19 * t596;
    let t9415 = 120.0_f64 * t9413 * t521;
    let t9417 = 1.0_f64 / t2490 / t182;
    let t9419 = t9417 * t9368 * t2495;
    (t9396, t9409, t9412, t9415, t9417, t9419)
}
