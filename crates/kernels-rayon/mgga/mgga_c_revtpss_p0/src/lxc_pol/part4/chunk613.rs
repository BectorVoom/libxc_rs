//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 613/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk613(t2918: f64, t935: f64, t915: f64, t913: f64, t275: f64) -> (f64, f64, f64, f64, f64) {
    let t2919 = t2918 * t935;
    let t2921 = 1.0_f64 * t915 * t2919;
    let t2922 = t913 * t913;
    let t2923 = 1.0_f64 / t2922;
    let t2924 = t275 * t2923;
    (t2919, t2921, t2922, t2923, t2924)
}
