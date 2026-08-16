//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1047/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1047(t11773: f64, t3114: f64, t1024: f64, t3230: f64, t11213: f64, t225: f64, t366: f64, t11223: f64, t1053: f64, t3223: f64, t3215: f64, t3224: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11774 = t3114 * t11773;
    let t11779 = t1024 * t3230;
    let t11782 = t11213 * t225;
    let t11783 = t11782 * t366;
    let t11788 = t11223 * t225;
    let t11792 = t3223 * t1053;
    let t11795 = t3224 * t3215;
    (t11774, t11779, t11782, t11783, t11788, t11792, t11795)
}
