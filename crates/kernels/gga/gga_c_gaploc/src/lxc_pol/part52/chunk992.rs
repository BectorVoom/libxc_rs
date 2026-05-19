//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 992/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk992<F: Float>(t1339: F, t14266: F, t14267: F, t1441: F, t1537: F, t41594: F, t41606: F, t41615: F, t41645: F, t46030: F, t46031: F, t46033: F, t46035: F, t46036: F, t46037: F, t46045: F, t46047: F, t46052: F, t46055: F, t46057: F, t47829: F, t47832: F, t590: F) -> F {
    let t50493 = -F::cast_from(0.51123901271894332901e1_f64) * t41594 - F::cast_from(0.38342925953920749676e1_f64) * t41606 - t46030 + t46031 + F::cast_from(0.63904876589867916127e-1_f64) * t41615 - t46033 + t46035 + t46036 + t46037 + F::cast_from(0.38342925953920749676e1_f64) * t41645 + t46045 + F::cast_from(0.51123901271894332902e0_f64) * t1441 * t14267 * t590 - F::cast_from(0.51123901271894332902e0_f64) * t1537 * t1339 * t14266 * t590 + F::cast_from(0.76685851907841499354e0_f64) * t47829 - F::cast_from(0.15337170381568299871e1_f64) * t47832 + F::cast_from(0.38342925953920749676e0_f64) * t46047 - t46052 + t46055 + t46057;
    t50493
}
