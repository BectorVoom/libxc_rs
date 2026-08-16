//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2702/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2702(t1409: f64, t1426: f64, t67: f64, t1434: f64, t16558: f64, t17635: f64, t1864: f64, t19322: f64, t19323: f64, t19331: f64, t19334: f64, t20218: f64, t20219: f64, t20222: f64, t31: f64, t3966: f64, t3997: f64, t5399: f64, t628: f64, t642: f64, t65: f64, t67060: f64, t70458: f64, t7445: f64, t80: f64) -> f64 {
    let t75361 = t1409 * t1426 * t67;
    let t75392 = -t19322 * t1864 * t16558 / 4.0_f64 - t75361 * t19323 / 2.0_f64 - t19322 * t7445 * t3966 / 2.0_f64 - t70458 * t65 * t80 / 12.0_f64 - t31 * t67060 * t65 * t80 / 12.0_f64 - t20218 * t628 * t80 / 12.0_f64 - t20219 * t642 / 12.0_f64 - t17635 * t1426 * t80 / 4.0_f64 - t19334 * t1426 * t80 / 4.0_f64 - t5399 * t3997 * t80 / 4.0_f64 - t20222 * t642 / 4.0_f64 - t19331 * t1434 / 4.0_f64;
    t75392
}
