//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1053/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1053(t104990: f64, t128418: f64, t128420: f64, t128422: f64, t128429: f64, t128438: f64, t128441: f64, t128443: f64, t128444: f64, t128449: f64, t128452: f64, t128454: f64, t128457: f64, t128460: f64, t2040: f64, t2165: f64, t28951: f64, t28952: f64, t29252: f64, t29855: f64, t652: f64, t7042: f64, t7266: f64, t8690: f64) -> f64 {
    let t130354 = -2.0_f64 * t2165 * t28951 * t652 - 2.0_f64 * t104990 * t2040 - 2.0_f64 * t28952 * t7266 + 6.0_f64 * t29252 * t8690 - 2.0_f64 * t29855 * t7042 - t128418 - t128420 - t128422 + t128429 + t128438 - t128441 - t128443 - t128444 - t128449 - t128452 - t128454 - t128457 - t128460;
    t130354
}
