//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 950/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk950(t1317: f64, t3853: f64, t1320: f64, t4029: f64, t1333: f64, t3863: f64, t27: f64, t583: f64, t521: f64, t19: f64, t596: f64, t182: f64, t2490: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9395 = t1317 * t3853;
    let t9398 = t1320 * t4029;
    let t9406 = t1317 * t4029;
    let t9408 = t3863 * t1333;
    let t9410 = t583 * t27;
    let t9411 = t9410 * t521;
    let t9413 = t19 * t596;
    let t9415 = 120.0_f64 * t9413 * t521;
    let t9417 = 1.0_f64 / t2490 / t182;
    (t9395, t9398, t9406, t9408, t9411, t9415, t9417)
}
