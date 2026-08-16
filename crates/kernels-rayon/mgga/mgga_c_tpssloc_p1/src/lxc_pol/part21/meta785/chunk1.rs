//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2720/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2720(t1307: f64, t1365: f64, t16018: f64, t16186: f64, t16192: f64, t16196: f64, t16199: f64, t19631: f64, t19708: f64, t19715: f64, t19716: f64, t19719: f64, t19724: f64, t225: f64, t3719: f64, t3734: f64, t3844: f64, t5272: f64, t5278: f64, t5279: f64, t5280: f64, t548: f64, t57193: f64, t57194: f64, t57196: f64, t57197: f64, t57200: f64, t57201: f64, t57217: f64, t57238: f64, t6330: f64, t6404: f64, t68: f64, t6924: f64) -> f64 {
    let t57266 = -48.0_f64 * t16186 * t19719 - 48.0_f64 * t5272 * t68 * t5280 - 48.0_f64 * t19708 * t16196 - 24.0_f64 * t5278 * t5279 * t16018 - (t57193 + t57194 + t57196 + t57197 + t57200 + t57201 + t57217 + t57238) * t225 * t548 - 360.0_f64 * t5278 * t6924 * t6330 * t3734 + 60.0_f64 * t5278 * t19715 * t3719 + 120.0_f64 * t16186 * t19716 - 24.0_f64 * t5278 * t1365 * t19631 * t1307 - 12.0_f64 * t5278 * t19724 * t3719 + 120.0_f64 * t19708 * t16192 - 24.0_f64 * t19708 * t16199 - 12.0_f64 * t6404 * t3844;
    t57266
}
