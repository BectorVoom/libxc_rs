//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2267/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2267<F: Float>(t13186: F, t13242: F, t16836: F, t2623: F, t2643: F, t41084: F, t41086: F, t41088: F, t4167: F, t46912: F, t46918: F, t46920: F, t46926: F, t46929: F, t46930: F, t46936: F, t9634: F, t9646: F, t9647: F, t9663: F) -> F {
    let t46938 = t46912 + F::cast_from(455.0_f64) / F::cast_from(216.0_f64) * t41084 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t41086 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t41088 + t16836 * t9634 / F::cast_from(512.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t46918 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t46920 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2643 * t9646 * t13242 * t9647 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t46926 - t46929 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t46930 - t4167 * t9663 / F::cast_from(3072.0_f64) - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t2623 * t13186 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t46936;
    t46938
}
