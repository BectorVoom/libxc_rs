//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2658/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2658(t12606: f64, t12695: f64, t12699: f64, t12702: f64, t1420: f64, t16558: f64, t19368: f64, t19369: f64, t19372: f64, t19377: f64, t19390: f64, t19397: f64, t2244: f64, t2250: f64, t2267: f64, t2274: f64, t39: f64, t39159: f64, t39168: f64, t3990: f64, t51: f64, t5392: f64, t5398: f64, t607: f64, t615: f64, t9287: f64, t9300: f64) -> f64 {
    let t55801 = 5.0_f64 / 9.0_f64 * t39 * t2267 * t16558 * t607 + 5.0_f64 / 18.0_f64 * t39 * t19377 * t2250 - 5.0_f64 / 108.0_f64 * t39 * t9287 * t5398 * t2244 - 80.0_f64 / 27.0_f64 * t1420 * t12699 - 40.0_f64 / 27.0_f64 * t1420 * t12702 - 20.0_f64 / 81.0_f64 * t1420 * t12695 + 5.0_f64 / 108.0_f64 * t51 * t19390 * t2250 + 5.0_f64 / 162.0_f64 * t51 * t39168 * t5392 * t2244 + 5.0_f64 / 9.0_f64 * t51 * t3990 * t12606 + 5.0_f64 / 9.0_f64 * t51 * t2274 * t16558 * t607 + 5.0_f64 / 18.0_f64 * t51 * t19397 * t2250 + 5.0_f64 / 108.0_f64 * t51 * t9300 * t5398 * t2244 - 80.0_f64 / 27.0_f64 * t615 * t19372 + 20.0_f64 / 81.0_f64 * t615 * t19369 - 5.0_f64 / 108.0_f64 * t39 * t19368 * t2250 + 5.0_f64 / 162.0_f64 * t39 * t39159 * t5392 * t2244;
    t55801
}
