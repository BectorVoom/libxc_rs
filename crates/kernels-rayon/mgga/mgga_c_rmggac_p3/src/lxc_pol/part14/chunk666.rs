//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 666/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk666(t1685: f64, t71: f64, t131: f64, t638: f64, t639: f64, t2338: f64, t356: f64, t2164: f64, t574: f64, t1656: f64, t640: f64, t2402: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8849 = t71 * t1685;
    let t8850 = t8849 * t131;
    let t8852 = t638 * t639 * t8850;
    let t8854 = t2338 * t356;
    let t8856 = t638 * t639 * t8854;
    let t8858 = t2164 * t574;
    let t8860 = t638 * t639 * t8858;
    let t8862 = t640 * t1656;
    let t8864 = t638 * t639 * t8862;
    let t8866 = t2402 * t333;
    (t8849, t8850, t8852, t8854, t8856, t8858, t8860, t8862, t8864, t8866)
}
