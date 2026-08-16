//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2651/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2651(t109: f64, t55530: f64, t55566: f64, t2363: f64, t576: f64, t12649: f64, t12652: f64, t12653: f64, t12656: f64, t12661: f64, t12708: f64, t1410: f64, t1426: f64, t1434: f64, t19343: f64, t19346: f64, t19349: f64, t19441: f64, t2304: f64, t3961: f64, t3962: f64, t3967: f64, t3997: f64, t4018: f64, t5403: f64, t609: f64, t642: f64, t80: f64) -> (f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t55568 = piecewise3(t110, 0.0_f64, t55530 + t55566);
    let t55571 = t576 * t2363;
    let t55631 = -t12652 * t1426 * t80 / 3.0_f64 - t3961 * t3997 * t80 / 3.0_f64 - t19343 * t642 / 3.0_f64 - t12661 * t1426 * t80 / 6.0_f64 - t3967 * t3997 * t80 / 3.0_f64 - t19346 * t642 / 3.0_f64 - t1410 * t12708 * t80 / 6.0_f64 - t19349 * t642 / 3.0_f64 - t5403 * t2304 / 6.0_f64 - t609 * t19441 / 6.0_f64 - t12649 * t1434 / 6.0_f64 - t12653 * t1434 / 3.0_f64 - t12656 * t1434 / 3.0_f64 - t3962 * t4018 / 3.0_f64;
    (t55568, t55571, t55631)
}
