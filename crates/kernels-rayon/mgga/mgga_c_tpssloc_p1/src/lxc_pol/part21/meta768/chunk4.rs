//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2657/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2657(t12606: f64, t12705: f64, t1420: f64, t19378: f64, t19381: f64, t2262: f64, t2267: f64, t2274: f64, t2275: f64, t2278: f64, t39: f64, t39210: f64, t3981: f64, t43: f64, t45970: f64, t45974: f64, t51: f64, t5408: f64, t5411: f64, t5416: f64, t55: f64, t55677: f64, t55716: f64, t55723: f64, t615: f64) -> f64 {
    let t55751 = -5.0_f64 / 27.0_f64 * t45970 * t55716 + 5.0_f64 / 27.0_f64 * t45974 * t55716 - t39210 + 220.0_f64 / 81.0_f64 * t2262 * t5408 + 5.0_f64 / 9.0_f64 * t39 * t2267 * t55723 + 220.0_f64 / 27.0_f64 * t2262 * t5411 - 40.0_f64 / 9.0_f64 * t615 * t19381 + 5.0_f64 / 6.0_f64 * t39 * t43 * t55677 - 220.0_f64 / 27.0_f64 * t5416 * t2278 + 220.0_f64 / 81.0_f64 * t5416 * t2275 + 40.0_f64 / 9.0_f64 * t1420 * t12705 - 5.0_f64 / 6.0_f64 * t51 * t55 * t55677 + 5.0_f64 / 9.0_f64 * t51 * t2274 * t55723 + 5.0_f64 / 9.0_f64 * t39 * t3981 * t12606 - 40.0_f64 / 27.0_f64 * t615 * t19378;
    t55751
}
