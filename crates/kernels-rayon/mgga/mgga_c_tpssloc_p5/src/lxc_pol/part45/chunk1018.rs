//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1018/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1018(t1307: f64, t1352: f64, t2085: f64, t22633: f64, t6976: f64, t1338: f64, t31584: f64, t3787: f64, t8617: f64, t114111: f64, t114115: f64, t114117: f64, t114119: f64, t114122: f64, t114127: f64, t1336: f64, t31636: f64, t31637: f64, t3777: f64, t3793: f64, t3851: f64) -> f64 {
    let t115484 = t22633 * t6976 * t2085 * t1307 * t1352;
    let t115486 = t1338 * t31584;
    let t115494 = t3787 * t8617;
    let t115498 = -t114111 + t114115 + 0.3289868133696452873e-1_f64 * t115484 - 2.0_f64 * t1336 * t115486 * t1352 - t1336 * t31636 * t3851 - 2.0_f64 * t3777 * t31637 - t114117 + t114119 + 2.0_f64 * t1336 * t115494 * t3793 + t114122 - t114127;
    t115498
}
