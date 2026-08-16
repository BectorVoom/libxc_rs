//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1254/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1254(t107225: f64, t3153: f64, t1976: f64, t6235: f64, t4746: f64, t7810: f64, t29834: f64, t7143: f64, t106655: f64, t994: f64, t29833: f64, t3056: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t107226 = t107225 * t3153;
    let t107240 = t6235 * t1976;
    let t107283 = t4746 * t7810;
    let t107286 = t29834 * t7143;
    let t107358 = t4746 * t7143;
    let t107435 = t994 * t106655;
    let t107496 = t29833 * t3056 * t7143;
    (t107226, t107240, t107283, t107286, t107358, t107435, t107496)
}
