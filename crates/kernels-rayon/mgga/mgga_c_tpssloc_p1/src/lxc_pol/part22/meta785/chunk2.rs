//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2705/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2705(t12698: f64, t1420: f64, t16558: f64, t19401: f64, t20217: f64, t20234: f64, t20235: f64, t20238: f64, t20241: f64, t20246: f64, t2274: f64, t39: f64, t39168: f64, t39210: f64, t3990: f64, t3994: f64, t43: f64, t51: f64, t5398: f64, t5416: f64, t55: f64, t607: f64, t615: f64, t621: f64, t67060: f64) -> f64 {
    let t75494 = 5.0_f64 / 162.0_f64 * t51 * t39168 * t20234 * t607 + 5.0_f64 / 6.0_f64 * t51 * t12698 * t5398 + 5.0_f64 / 6.0_f64 * t51 * t3990 * t16558 + 5.0_f64 / 18.0_f64 * t51 * t2274 * t20217 * t607 - 20.0_f64 / 9.0_f64 * t615 * t20238 + 10.0_f64 / 81.0_f64 * t615 * t20235 - 20.0_f64 / 9.0_f64 * t615 * t20241 + 5.0_f64 / 6.0_f64 * t39 * t43 * t67060 + 3080.0_f64 / 81.0_f64 * t20246 * t621 - 220.0_f64 / 9.0_f64 * t5416 * t3994 + 20.0_f64 / 3.0_f64 * t1420 * t19401 - 5.0_f64 / 6.0_f64 * t51 * t55 * t67060 - t39210;
    t75494
}
