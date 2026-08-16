//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 486/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk486(t1228: f64, t1904: f64, t1184: f64, t1888: f64, t476: f64, t221: f64, t1867: f64, t4522: f64, t6108: f64, t1475: f64, t1494: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6125 = t1228 * t1904;
    let t6129 = t1888 * t1184;
    let t6130 = t6129 * t476;
    let t6131 = t221 * t6130;
    let t6134 = t1867 * t4522;
    let t6135 = t6134 * t476;
    let t6136 = t221 * t6135;
    let t6139 = t6108 * t476;
    let t6140 = t221 * t6139;
    let t6144 = t1475 * t1494;
    let t6145 = t221 * t6144;
    let t6148 = t1867 * t476;
    let t6149 = t6148 * t209;
    (t6125, t6130, t6131, t6135, t6136, t6139, t6140, t6144, t6145, t6149)
}
