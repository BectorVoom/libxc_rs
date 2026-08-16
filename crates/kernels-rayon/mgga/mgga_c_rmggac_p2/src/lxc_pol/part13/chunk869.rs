//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 869/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk869(t1327: f64, t574: f64, t640: f64, t7323: f64, t1243: f64, t236: f64, t3351: f64, t618: f64, t9210: f64, t7248: f64, t833: f64, t1614: f64, t1971: f64, t495: f64, t511: f64, t7230: f64) -> (f64, f64, f64, f64) {
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39350 = t3351 * t9210 * t236 * t618 * t1243;
    let t39355 = t3351 * t7248 * t236 * t618 * t833;
    let t39360 = t7230 * t1971 * t511 * t1614 * t495;
    (t39345, t39350, t39355, t39360)
}
