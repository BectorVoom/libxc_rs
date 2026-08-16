//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2675/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2675<F: Float>(t12156: F, t12157: F, t12161: F, t12303: F, t1307: F, t1345: F, t1365: F, t16018: F, t16186: F, t16191: F, t16192: F, t16195: F, t16202: F, t1799: F, t1819: F, t19708: F, t1995: F, t3719: F, t3734: F, t3839: F, t3844: F, t5187: F, t5272: F, t5278: F, t5280: F, t68: F, t6924: F) -> F {
    let t54525 = -F::cast_from(360.0_f64) * t12156 * t1799 * t5278 * t6924 - F::cast_from(36.0_f64) * t1307 * t1365 * t16018 * t5278 + F::cast_from(180.0_f64) * t1995 * t3734 * t5187 * t5278 + F::cast_from(180.0_f64) * t12303 * t16191 * t5278 - F::cast_from(36.0_f64) * t16195 * t3719 * t5278 - F::cast_from(36.0_f64) * t3839 * t5280 * t68 + F::cast_from(60.0_f64) * t12157 * t1819 - F::cast_from(36.0_f64) * t12161 * t19708 + F::cast_from(9.0_f64) * t1345 * t16202 + F::cast_from(180.0_f64) * t16186 * t16192 - F::cast_from(36.0_f64) * t3844 * t5272;
    t54525
}
