//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2271/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2271(t1459: f64, t28277: f64, t28280: f64, t5795: f64, t7331: f64, t28268: f64, t116: f64, t28042: f64, t572: f64, t670: f64, t2371: f64, t28276: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101613 = 12.0_f64 * t1459 * t28277;
    let t101617 = 6.0_f64 * t1459 * t28280;
    let t101619 = 12.0_f64 * t5795 * t7331;
    let t101621 = 12.0_f64 * t1459 * t28268;
    let t101622 = t116 * t28042;
    let t101625 = 12.0_f64 * t572 * t101622 * t670;
    let t101628 = 6.0_f64 * t572 * t28276 * t2371;
    (t101613, t101617, t101619, t101621, t101625, t101628)
}
