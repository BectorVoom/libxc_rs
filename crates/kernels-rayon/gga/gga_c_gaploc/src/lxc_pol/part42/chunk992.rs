//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 992/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk992(t1339: f64, t14266: f64, t14267: f64, t1441: f64, t1537: f64, t41594: f64, t41606: f64, t41615: f64, t41645: f64, t46030: f64, t46031: f64, t46033: f64, t46035: f64, t46036: f64, t46037: f64, t46045: f64, t46047: f64, t46052: f64, t46055: f64, t46057: f64, t47829: f64, t47832: f64, t590: f64) -> f64 {
    let t50493 = -0.51123901271894332901e1_f64 * t41594 - 0.38342925953920749676e1_f64 * t41606 - t46030 + t46031 + 0.63904876589867916127e-1_f64 * t41615 - t46033 + t46035 + t46036 + t46037 + 0.38342925953920749676e1_f64 * t41645 + t46045 + 0.51123901271894332902e0_f64 * t1441 * t14267 * t590 - 0.51123901271894332902e0_f64 * t1537 * t1339 * t14266 * t590 + 0.76685851907841499354e0_f64 * t47829 - 0.15337170381568299871e1_f64 * t47832 + 0.38342925953920749676e0_f64 * t46047 - t46052 + t46055 + t46057;
    t50493
}
