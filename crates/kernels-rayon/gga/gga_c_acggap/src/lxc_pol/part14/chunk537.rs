//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 537/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk537(t3669: f64, t381: f64, t390: f64, t1015: f64, t144: f64, t377: f64, t951: f64, t409: f64, t1032: f64, t1113: f64, t1108: f64, t360: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3700 = t381 * t3669;
    let t3702 = 0.17006693853500995666e-1_f64 * t3700 * t390;
    let t3706 = 1.0_f64 / t1015 / t144;
    let t3740 = t377 * t951;
    let t3741 = t3740 * t409;
    let t3745 = t1032 * t1113;
    let t3752 = t1032 * t1108;
    let t3754 = t879 * t360;
    (t3702, t3706, t3740, t3741, t3745, t3752, t3754)
}
