//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 979/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk979<F: Float>(t16236: F, t8532: F, t322: F, t1114: F, t16241: F, t8482: F, t1027: F, t3107: F, t123: F, t1239: F, t15311: F, t424: F, t444: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
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
    let t17926 = F::cast_from(1.0_f64) / t424 / t444;
    (t17897, t17898, t17903, t17904, t17907, t17908, t17919, t17920, t17921, t17922, t17926)
}
