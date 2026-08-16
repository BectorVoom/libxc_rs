//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1017/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1017(t28823: f64, t8607: f64, t114360: f64, t127122: f64, t127124: f64, t127125: f64, t128438: f64, t128441: f64, t128443: f64, t128444: f64, t128449: f64, t128452: f64, t128454: f64, t128457: f64, t128460: f64, t128464: f64, t28969: f64, t29201: f64, t29247: f64, t29380: f64, t8450: f64) -> f64 {
    let t128466 = 2.0_f64 * t8607 * t28823;
    let t128469 = -6.0_f64 * t114360 * t29247 + 3.0_f64 * t28969 * t8450 - 2.0_f64 * t29201 * t8450 + 6.0_f64 * t29380 * t8450 - t127122 - t127124 - t127125 + t128438 - t128441 - t128443 - t128444 - t128449 - t128452 - t128454 - t128457 - t128460 + t128464 + t128466;
    t128469
}
