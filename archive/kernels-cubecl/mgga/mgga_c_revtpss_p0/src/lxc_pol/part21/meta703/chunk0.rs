//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2526/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2526<F: Float>(t39454: F, t521: F, t1333: F, t9413: F, t30: F, t513: F, t9603: F, t33: F, t516: F, t9615: F, t10008: F, t213: F) -> (F, F, F, F, F) {
    let t46291 = t39454 * t521;
    let t46297 = F::cast_from(480.0_f64) * t9413 * t1333;
    let t46310 = F::cast_from(1.0_f64) / t513 / t9603 / t30;
    let t46328 = F::cast_from(1.0_f64) / t516 / t9615 / t33;
    let t46350 = t213 * t10008;
    (t46291, t46297, t46310, t46328, t46350)
}
