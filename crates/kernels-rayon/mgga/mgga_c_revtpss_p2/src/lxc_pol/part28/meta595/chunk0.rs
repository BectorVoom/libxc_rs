//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2067/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2067(t2453: f64, t26053: f64, t9676: f64, t4078: f64, t689: f64, t7242: f64, t1358: f64, t2439: f64, t7274: f64, t785: f64, t26064: f64, t3920: f64) -> (f64, f64, f64, f64, f64) {
    let t94725 = t2453 * t26053;
    let t94726 = t94725 * t9676;
    let t94729 = t689 * t7242 * t4078;
    let t94733 = t2439 * t785 * t7274 * t1358;
    let t94735 = t26064 * t3920;
    (t94725, t94726, t94729, t94733, t94735)
}
