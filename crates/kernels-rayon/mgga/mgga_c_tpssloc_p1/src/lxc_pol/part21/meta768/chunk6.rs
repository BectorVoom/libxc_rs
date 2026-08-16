//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2659/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2659(t12595: f64, t12606: f64, t12609: f64, t12652: f64, t16558: f64, t19420: f64, t19425: f64, t19430: f64, t19435: f64, t2244: f64, t2250: f64, t2291: f64, t2298: f64, t39096: f64, t39114: f64, t4007: f64, t4012: f64, t5392: f64, t5398: f64, t55677: f64, t55723: f64, t607: f64, t634: f64, t638: f64, t9321: f64, t9330: f64) -> f64 {
    let t55867 = 3640.0_f64 / 81.0_f64 * t39096 * t5392 * t2244 - 1120.0_f64 / 27.0_f64 * t12595 * t12652 - 280.0_f64 / 27.0_f64 * t19420 * t2250 + 56.0_f64 / 9.0_f64 * t2291 * t55723 + 56.0_f64 / 9.0_f64 * t4007 * t12606 - 280.0_f64 / 27.0_f64 * t9321 * t5398 * t2244 + 56.0_f64 / 9.0_f64 * t2291 * t16558 * t607 + 28.0_f64 / 9.0_f64 * t19425 * t2250 - 4.0_f64 / 3.0_f64 * t634 * t55677 + 3640.0_f64 / 81.0_f64 * t39114 * t5392 * t2244 + 1120.0_f64 / 27.0_f64 * t12609 * t12652 + 280.0_f64 / 27.0_f64 * t19430 * t2250 + 56.0_f64 / 9.0_f64 * t2298 * t55723 + 56.0_f64 / 9.0_f64 * t4012 * t12606 + 280.0_f64 / 27.0_f64 * t9330 * t5398 * t2244 + 56.0_f64 / 9.0_f64 * t2298 * t16558 * t607 + 28.0_f64 / 9.0_f64 * t19435 * t2250 + 4.0_f64 / 3.0_f64 * t638 * t55677;
    t55867
}
