//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2453/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2453(t11045: f64, t42332: f64, t42340: f64, t42341: f64, t43288: f64, t23508: f64, t43292: f64, t10163: f64, t386: f64, t68: f64, t3215: f64, t3399: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43562 = t42332 * t11045;
    let t43576 = t42340 * t42341 * t43288;
    let t43577 = t23508 * t43292;
    let t43603 = 1.0_f64 / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0_f64 / t43636;
    let t43688 = t3399 * t3399;
    (t43562, t43576, t43577, t43604, t43637, t43688)
}
