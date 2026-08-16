//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 885/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk885(t1591: f64, t2039: f64, t270: f64, t638: f64, t2338: f64, t7323: f64, t7324: f64, t1327: f64, t574: f64, t640: f64, t1243: f64, t236: f64, t3351: f64, t618: f64, t9210: f64) -> (f64, f64, f64, f64) {
    let t39338 = t638 * t2039 * t1591 * t270;
    let t39339 = 0.30487649791575028314e-3_f64 * t39338;
    let t39341 = t7323 * t2338 * t7324;
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39350 = t3351 * t9210 * t236 * t618 * t1243;
    (t39339, t39341, t39345, t39350)
}
