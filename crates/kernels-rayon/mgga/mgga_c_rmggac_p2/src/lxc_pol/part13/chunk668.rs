//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 668/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk668(t638: f64, t639: f64, t8854: f64, t2164: f64, t574: f64, t1656: f64, t640: f64, t2298: f64, t4601: f64, t2301: f64, t2604: f64, t1614: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8856 = t638 * t639 * t8854;
    let t8858 = t2164 * t574;
    let t8860 = t638 * t639 * t8858;
    let t8862 = t640 * t1656;
    let t8864 = t638 * t639 * t8862;
    let t8872 = t4601 * t2298;
    let t8881 = t2604 * t2301;
    let t8884 = t645 * t1614;
    (t8856, t8858, t8860, t8862, t8864, t8872, t8881, t8884)
}
