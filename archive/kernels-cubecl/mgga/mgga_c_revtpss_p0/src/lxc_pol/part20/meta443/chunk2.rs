//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1696/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1696<F: Float>(t10208: F, t10213: F, t10254: F, t2339: F, t2340: F, t2366: F, t46143: F, t46144: F, t46146: F, t46148: F, t46150: F, t46152: F, t46154: F, t46157: F, t46158: F, t46166: F, t46228: F, t655: F, t69: F) -> F {
    let t46232 = t46143 + F::cast_from(616.0_f64) / F::cast_from(27.0_f64) * t46144 + F::cast_from(44.0_f64) / F::cast_from(3.0_f64) * t46146 - F::cast_from(22.0_f64) / F::cast_from(3.0_f64) * t46148 + F::cast_from(8.0_f64) * t46150 - F::cast_from(8.0_f64) * t46152 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t46154 + F::cast_from(3.0_f64) * t69 * t46157 * t46158 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t69 * t10208 * t2340 * t2366 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t69 * t2339 * t46166 + t69 * t10213 * t10254 - t69 * t655 * t46228 / F::cast_from(8.0_f64);
    t46232
}
