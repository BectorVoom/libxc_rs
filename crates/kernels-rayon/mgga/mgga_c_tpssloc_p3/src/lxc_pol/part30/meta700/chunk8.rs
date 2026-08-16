//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2261/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2261(t17092: f64, t25200: f64, t2718: f64, t4147: f64, t4300: f64, t6663: f64, t7537: f64, t82209: f64, t82211: f64, t82219: f64, t855: f64, t87805: f64, t98927: f64, t98932: f64, t98941: f64, t98945: f64) -> f64 {
    let t98947 = 0.82246703342411321825e-2_f64 * t98927 - 2.0_f64 * t17092 * t6663 - t87805 - 0.12793931631041761173e0_f64 * t82209 + 0.38381794893125283518e-1_f64 * t98932 - 0.63969658155208805863e-1_f64 * t82211 + 4.0_f64 * t855 * t2718 * t7537 * t4300 + 4.0_f64 * t4147 * t25200 - t82219 - 0.76763589786250567037e-1_f64 * t98941 - 0.82246703342411321825e-2_f64 * t98945;
    t98947
}
