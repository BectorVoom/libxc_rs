//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 684/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk684(t342: f64, t4980: f64, t3154: f64, t3302: f64, t3316: f64, t378: f64, t1043: f64, t357: f64, t198: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4995 = t3316 * t378;
    let t4996 = t342 * t4995;
    let t4997 = t3302 * t1043;
    let t4998 = t4997 * t357;
    let t5023 = t198 * t336;
    (t4981, t4982, t4995, t4996, t4998, t5023)
}
