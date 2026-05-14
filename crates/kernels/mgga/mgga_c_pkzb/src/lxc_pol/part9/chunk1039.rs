//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1039/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1039<F: Float>(t1429: F, t1435: F, t444: F, t2503: F, t500: F, t23: F, t2499: F, t4819: F, t1419: F, t19418: F, t19427: F, t19435: F, t19439: F, t19444: F, t19446: F, t2490: F, t2494: F, t434: F, t4816: F, t6655: F, t6662: F, t6668: F, t7: F, t980: F) -> (F, F, F, F) {
    let t19450 = t1435 * t1429 * t444;
    let t19453 = t2503 * t500;
    let t19455 = 20.0 * t23 * t19453;
    let t19458 = t2499 * t4819;
    let t19461 = 10.0 * t23 * t19418 + 880.0 / 27.0 * t1419 * t2490 + 440.0 / 9.0 * t1419 * t2494 - 80.0 / 9.0 * t434 * t6662 + 10.0 / 9.0 * t7 * t19427 + 80.0 / 27.0 * t434 * t6655 + 40.0 * t434 * t6668 + 40.0 / 81.0 * t7 * t19435 + 10.0 / 3.0 * t7 * t19439 + t19444 + 40.0 / 81.0 * t23 * t19446 - 10.0 / 3.0 * t23 * t19450 - t19455 - 80.0 / 9.0 * t980 * t4816 + 10.0 / 9.0 * t23 * t19458;
    (t19450, t19453, t19458, t19461)
}
