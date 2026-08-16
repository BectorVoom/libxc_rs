//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1048/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1048(t4975: f64, t999: f64, t4781: f64, t3298: f64, t378: f64, t342: f64, t3154: f64, t3302: f64, t1043: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4976 = t4975 * t999;
    let t4977 = t4781 * t4976;
    let t4980 = t3298 * t378;
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4983 = t4982 * t1043;
    (t4976, t4977, t4980, t4981, t4982, t4983)
}
