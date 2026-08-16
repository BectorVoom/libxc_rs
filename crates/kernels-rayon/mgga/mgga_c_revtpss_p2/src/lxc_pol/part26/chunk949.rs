//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 949/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk949(t11291: f64, t11293: f64, t11296: f64, t11303: f64, t11382: f64, t11390: f64, t11392: f64, t11394: f64, t11590: f64, t11593: f64, t11596: f64, t11600: f64, t11604: f64) -> f64 {
    let t12199 = t11291 + t11293 + t11296 - t11303 + t11382 + t11390 + t11604 - t11392 - t11394 - t11593 + t11596 - t11600 + t11590;
    t12199
}
