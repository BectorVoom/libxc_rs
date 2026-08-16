//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 890/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk890(t4893: f64, t4983: f64, t1071: f64, t1089: f64, t1668: f64, t378: f64, t4866: f64, t3316: f64, t342: f64, t1043: f64, t3302: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4984 = t4893 * t4983;
    let t4988 = t1071 * t1668 * t1089;
    let t4992 = t378 * t4866 * t1089;
    let t4995 = t3316 * t378;
    let t4996 = t342 * t4995;
    let t4997 = t3302 * t1043;
    let t4998 = t4997 * t357;
    (t4984, t4988, t4992, t4995, t4996, t4998)
}
