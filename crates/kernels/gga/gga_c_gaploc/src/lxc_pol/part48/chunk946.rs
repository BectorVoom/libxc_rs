//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 946/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk946<F: Float>(t41588: F, t41592: F, t41594: F, t41606: F, t41615: F, t41621: F, t41631: F, t41636: F, t41640: F, t41643: F, t41645: F, t46030: F, t46031: F, t46033: F, t46035: F, t46036: F, t46037: F, t46045: F, t46047: F) -> F {
    let t46049 = F::cast_from(0.38342925953920749677e1_f64) * t41588 - F::cast_from(0.23005755572352449806e1_f64) * t41592 - F::cast_from(0.51123901271894332903e1_f64) * t41594 - F::cast_from(0.38342925953920749677e1_f64) * t41606 - t46030 + t46031 + F::cast_from(0.63904876589867916128e-1_f64) * t41615 - t46033 + F::cast_from(0.11916829983950142223e0_f64) * t41621 + t46035 + t46036 + t46037 + F::cast_from(0.76685851907841499353e0_f64) * t41631 + F::cast_from(0.76685851907841499353e0_f64) * t41636 - F::cast_from(0.17041300423964777634e0_f64) * t41640 - F::cast_from(0.59584149919750711116e-1_f64) * t41643 + F::cast_from(0.38342925953920749677e1_f64) * t41645 + t46045 + F::cast_from(0.38342925953920749677e0_f64) * t46047;
    t46049
}
