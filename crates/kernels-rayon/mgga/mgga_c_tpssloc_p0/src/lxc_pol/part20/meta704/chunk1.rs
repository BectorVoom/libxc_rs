//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2675/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2675(t12156: f64, t12157: f64, t12161: f64, t12303: f64, t1307: f64, t1345: f64, t1365: f64, t16018: f64, t16186: f64, t16191: f64, t16192: f64, t16195: f64, t16202: f64, t1799: f64, t1819: f64, t19708: f64, t1995: f64, t3719: f64, t3734: f64, t3839: f64, t3844: f64, t5187: f64, t5272: f64, t5278: f64, t5280: f64, t68: f64, t6924: f64) -> f64 {
    let t54525 = -360.0_f64 * t12156 * t1799 * t5278 * t6924 - 36.0_f64 * t1307 * t1365 * t16018 * t5278 + 180.0_f64 * t1995 * t3734 * t5187 * t5278 + 180.0_f64 * t12303 * t16191 * t5278 - 36.0_f64 * t16195 * t3719 * t5278 - 36.0_f64 * t3839 * t5280 * t68 + 60.0_f64 * t12157 * t1819 - 36.0_f64 * t12161 * t19708 + 9.0_f64 * t1345 * t16202 + 180.0_f64 * t16186 * t16192 - 36.0_f64 * t3844 * t5272;
    t54525
}
