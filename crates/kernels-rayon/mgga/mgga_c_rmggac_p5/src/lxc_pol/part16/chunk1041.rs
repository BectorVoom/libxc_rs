//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1041/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1041(t1951: f64, t2039: f64, t270: f64, t638: f64, t1956: f64, t2046: f64, t2050: f64, t31: f64, t1954: f64, t5055: f64, t9008: f64, t46526: f64, t7192: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47698 = t638 * t2039 * t1951 * t270;
    let t47702 = t638 * t2039 * t1956 * t270;
    let t47706 = t2046 * t2050 * t1956 * t31;
    let t47710 = t638 * t2039 * t1954 * t270;
    let t47714 = t2046 * t2050 * t1954 * t31;
    let t47719 = t5055 * t9008;
    let t47721 = t7192 * t46526;
    (t47698, t47702, t47706, t47710, t47714, t47719, t47721)
}
