//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 510/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk510(t3298: f64, t378: f64, t342: f64, t3154: f64, t3302: f64, t3316: f64, t1678: f64, t359: f64, t198: f64, t336: f64, t1716: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4980 = t3298 * t378;
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4995 = t3316 * t378;
    let t4996 = t342 * t4995;
    let t5004 = t359 * t1678;
    let t5023 = t198 * t336;
    let t5044 = t689 * t1716;
    (t4980, t4981, t4982, t4995, t4996, t5004, t5023, t5044)
}
