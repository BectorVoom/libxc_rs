//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 805/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk805(t2305: f64, t35326: f64, t638: f64, t7292: f64, t8475: f64, t1591: f64, t2039: f64, t270: f64, t2338: f64, t7323: f64, t7324: f64, t1327: f64, t574: f64, t640: f64) -> (f64, f64, f64, f64, f64) {
    let t39308 = t35326 * t2305;
    let t39333 = t638 * t7292 * t8475;
    let t39338 = t638 * t2039 * t1591 * t270;
    let t39341 = t7323 * t2338 * t7324;
    let t39345 = t7323 * t640 * t574 * t1327;
    (t39308, t39333, t39338, t39341, t39345)
}
