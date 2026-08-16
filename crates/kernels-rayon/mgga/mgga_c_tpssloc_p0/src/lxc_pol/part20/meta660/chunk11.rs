//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2475/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2475(t1068: f64, t3209: f64, t13666: f64, t14667: f64, t4700: f64, t49228: f64, t49544: f64, t49548: f64, t49550: f64, t49552: f64, t49556: f64, t49558: f64, t49560: f64, t49562: f64) -> f64 {
    let t50775 = t3209 * t1068;
    let t50779 = -3.0_f64 * t13666 * t3209 * t4700 + 6.0_f64 * t14667 * t4700 * t50775 + t49228 - t49544 + t49548 - t49550 - t49552 - t49556 - t49558 + t49560 - t49562;
    t50779
}
