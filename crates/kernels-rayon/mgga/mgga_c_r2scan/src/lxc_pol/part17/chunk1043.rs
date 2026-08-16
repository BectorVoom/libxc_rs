//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1043/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1043(t2530: f64, t910: f64, t2526: f64, t920: f64, t2719: f64, t938: f64, t3055: f64, t6363: f64, t3053: f64, t6212: f64, t3090: f64, t3056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28320 = t910 * t2530;
    let t28325 = t2526 * t920;
    let t28335 = t2719 * t920;
    let t28390 = t938 * t2530;
    let t28404 = t3055 * t6363;
    let t29126 = t6212 * t3053;
    let t29177 = t6212 * t3090;
    let t29194 = t6212 * t3056;
    (t28320, t28325, t28335, t28390, t28404, t29126, t29177, t29194)
}
