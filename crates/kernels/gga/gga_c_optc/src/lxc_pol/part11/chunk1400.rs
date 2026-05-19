//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1400/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1400<F: Float>(t27203: F, t59022: F, t1: F, t11937: F, t11943: F, t11982: F, t12026: F, t1239: F, t15240: F, t15642: F, t17348: F, t17670: F, t17869: F, t17937: F, t17941: F, t17942: F, t27202: F, t285: F, t3103: F, t3116: F, t3119: F, t3235: F, t35932: F, t431: F, t4336: F, t4386: F, t4387: F, t450: F, t46193: F, t46469: F, t46729: F, t46733: F, t53963: F, t54066: F, t54109: F, t54120: F, t54141: F, t54174: F, t55768: F, t58350: F, t58358: F, t58547: F, t58932: F, t59004: F, t9128: F, t9175: F, t935: F, sigma2: F) -> (F, F) {
    let t59023 = t59022 * t27203;
    let t59028 = -F::cast_from(0.36629113921839320676e2_f64) * t54109 - t54120 / F::new(27.0) + F::cast_from(0.5680050638253047068e0_f64) * t3116 * t4336 * t3119 * t17348 + F::cast_from(0.36629113921839320676e2_f64) * t3103 * t54066 * t17941 + F::cast_from(0.1420012659563261767e0_f64) * t3116 * t15240 * t55768 - F::cast_from(0.25244669503346875858e1_f64) * t12026 * t17937 + F::new(1309.0) / F::new(486.0) * sigma2 * t58547 * t285 * t431 + F::cast_from(0.19535527424980971027e3_f64) * t54141 - F::cast_from(0.28345352648723563785e5_f64) * t9128 * t53963 * t46729 + F::cast_from(0.47242254414539272975e4_f64) * t11943 * t53963 * t46733 + F::cast_from(0.28345352648723563784e5_f64) * t9175 * t53963 * t46193 * t1239 * t935 - F::cast_from(0.15146801702008125515e1_f64) * t12026 * t17670 - F::cast_from(0.94667510637550784466e0_f64) * t3116 * t35932 * t59004 - F::cast_from(0.5860658227494291308e3_f64) * t11937 * t17942 + F::cast_from(0.65198711173415683908e-1_f64) * t4386 * t3235 * t58358 + F::cast_from(0.23181763972770020945e0_f64) * t15642 * t17869 - F::cast_from(0.10866451862235947318e0_f64) * t4386 * t4387 * t58350 + F::cast_from(0.31555836879183594822e0_f64) * t54174 + F::cast_from(0.47333755318775392234e0_f64) * t11982 * t46469 * t58932 + F::cast_from(0.56296038352410615326e5_f64) * t27202 * t450 * t59023 * t1;
    (t59023, t59028)
}
