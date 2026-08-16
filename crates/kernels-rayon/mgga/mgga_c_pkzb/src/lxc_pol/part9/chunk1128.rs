//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1128/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1128(t1419: f64, t19418: f64, t19427: f64, t19435: f64, t19439: f64, t19444: f64, t19446: f64, t19450: f64, t19455: f64, t19458: f64, t23: f64, t2490: f64, t2494: f64, t434: f64, t4816: f64, t6655: f64, t6662: f64, t6668: f64, t7: f64, t980: f64) -> f64 {
    let t19461 = 10.0_f64 * t23 * t19418 + 880.0_f64 / 27.0_f64 * t1419 * t2490 + 440.0_f64 / 9.0_f64 * t1419 * t2494 - 80.0_f64 / 9.0_f64 * t434 * t6662 + 10.0_f64 / 9.0_f64 * t7 * t19427 + 80.0_f64 / 27.0_f64 * t434 * t6655 + 40.0_f64 * t434 * t6668 + 40.0_f64 / 81.0_f64 * t7 * t19435 + 10.0_f64 / 3.0_f64 * t7 * t19439 + t19444 + 40.0_f64 / 81.0_f64 * t23 * t19446 - 10.0_f64 / 3.0_f64 * t23 * t19450 - t19455 - 80.0_f64 / 9.0_f64 * t980 * t4816 + 10.0_f64 / 9.0_f64 * t23 * t19458;
    t19461
}
