//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 754/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk754(t11544: f64, t11576: f64, t11609: f64, t11636: f64, t10777: f64, t11491: f64, t11493: f64, t11495: f64, t11496: f64, t11500: f64, t11503: f64, t11507: f64, t11510: f64, t1689: f64, t1809: f64, t1860: f64, t4794: f64, t5089: f64, t5172: f64, t604: f64, t702: f64) -> (f64, f64) {
    let t11638 = t11544 + t11576 + t11609 + t11636;
    let t11645 = -0.28111840756657074597e-1_f64 * t11491 - 0.42167761134985611897e-1_f64 * t11493 - 0.14055920378328537299e-1_f64 * t11495 * t11496 - 0.28111840756657074597e-1_f64 * t5089 * t11500 + 0.14055920378328537299e-1_f64 * t5089 * t11503 + 0.14055920378328537299e-1_f64 * t1809 * t11507 + 0.14055920378328537299e-1_f64 * t1809 * t11510 - t604 * t11638 - 3.0_f64 * t1689 * t5172 - 3.0_f64 * t4794 * t1860 - t10777 * t702;
    (t11638, t11645)
}
