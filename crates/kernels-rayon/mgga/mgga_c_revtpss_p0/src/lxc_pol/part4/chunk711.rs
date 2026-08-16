//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 711/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk711(t1211: f64, t3584: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3528: f64, t3530: f64, t3533: f64, t3537: f64, t3541: f64, t3545: f64) -> (f64, f64) {
    let t3585 = t1211 * t3584;
    let t3588 = -t3378 + t3381 - t3388 + t3430 + t3438 + t3528 + t3530 - t3533 + t3537 - t3541 - t3545;
    (t3585, t3588)
}
