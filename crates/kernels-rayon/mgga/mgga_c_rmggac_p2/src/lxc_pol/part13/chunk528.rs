//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 528/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk528(t2069: f64, t352: f64, t262: f64, t7204: f64, t2160: f64, t2165: f64, t638: f64, t2169: f64, t1288: f64, t71: f64, t131: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7205 = t2069 * t352;
    let t7206 = t262 * t7205;
    let t7207 = t7204 * t7206;
    let t7210 = t638 * t2160 * t2165;
    let t7213 = t638 * t2160 * t2169;
    let t7215 = t71 * t1288;
    let t7216 = t7215 * t131;
    let t7218 = t638 * t639 * t7216;
    (t7205, t7206, t7207, t7210, t7213, t7215, t7216, t7218)
}
