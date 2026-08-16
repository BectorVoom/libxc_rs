//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 936/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk936(t1805: f64, t5165: f64, t15068: f64, t5062: f64, t10796: f64, t6717: f64, t3474: f64, t6697: f64, t19630: f64, t3338: f64, t3337: f64, t5096: f64, t5172: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19918 = t5165 * t1805;
    let t19920 = t15068 * t5062;
    let t19922 = t10796 * t6717;
    let t19924 = t3474 * t6697;
    let t19926 = t3338 * t19630;
    let t19927 = t3337 * t19926;
    let t19929 = t5172 * t5096;
    (t19918, t19920, t19922, t19924, t19926, t19927, t19929)
}
