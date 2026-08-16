//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1037/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1037(t122537: f64, t1799: f64, t6637: f64, t6888: f64, t31618: f64, t6347: f64, t114064: f64, t115433: f64, t115494: f64, t127361: f64, t127362: f64, t127371: f64, t127375: f64, t127381: f64, t1336: f64, t31636: f64, t6388: f64, t6415: f64) -> f64 {
    let t128847 = t6888 * t6637 * t122537 * t1799;
    let t128851 = t6888 * t6637 * t31618 * t6347;
    let t128855 = t127361 + 2.0_f64 * t1336 * t115494 * t6388 + t127362 - t114064 - 0.3289868133696452873e-1_f64 * t128847 - 0.16449340668482264365e-1_f64 * t128851 - t1336 * t31636 * t6415 - t127371 - t127375 + t127381 + t115433;
    t128855
}
