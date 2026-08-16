//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 729/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk729(t2165: f64, t638: f64, t7184: f64, t2169: f64, t1343: f64, t7321: f64, t1327: f64, t4765: f64, t640: f64, t7352: f64, t1295: f64, t1302: f64, t131: f64, t20: f64, t2018: f64, t2020: f64, t252: f64) -> (f64, f64, f64, f64) {
    let t34662 = t638 * t7184 * t2165;
    let t34665 = t638 * t7184 * t2169;
    let t34683 = t7321 * t1343;
    let t34687 = t4765 * t34683 * t640 * t7352 * t1327;
    let t34704 = t1295 * t1302 * t20 * t2018 * t2020 * t640 * t131 * t252;
    (t34662, t34665, t34687, t34704)
}
