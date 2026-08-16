//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 979/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk979(t16236: f64, t8532: f64, t322: f64, t1114: f64, t16241: f64, t8482: f64, t1027: f64, t3107: f64, t123: f64, t1239: f64, t15311: f64, t424: f64, t444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17897 = t8532 * t16236;
    let t17898 = t322 * t17897;
    let t17903 = t1114 * t16241;
    let t17904 = t322 * t17903;
    let t17907 = t8482 * t16236;
    let t17908 = t322 * t17907;
    let t17919 = t3107 * t1027;
    let t17920 = t1239 * t123;
    let t17921 = t17919 * t17920;
    let t17922 = t15311 * t17921;
    let t17926 = 1.0_f64 / t424 / t444;
    (t17897, t17898, t17903, t17904, t17907, t17908, t17919, t17920, t17921, t17922, t17926)
}
