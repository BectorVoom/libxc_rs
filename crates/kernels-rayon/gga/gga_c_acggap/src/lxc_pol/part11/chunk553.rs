//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 553/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk553(t3732: f64, t384: f64, t377: f64, t951: f64, t409: f64, t1086: f64, t997: f64, t1032: f64, t1113: f64, t1092: f64, t1098: f64, t1108: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3733 = t384 * t3732;
    let t3740 = t377 * t951;
    let t3741 = t3740 * t409;
    let t3743 = t997 * t1086;
    let t3745 = t1032 * t1113;
    let t3747 = t997 * t1092;
    let t3750 = t997 * t1098;
    let t3752 = t1032 * t1108;
    (t3733, t3740, t3741, t3743, t3745, t3747, t3750, t3752)
}
