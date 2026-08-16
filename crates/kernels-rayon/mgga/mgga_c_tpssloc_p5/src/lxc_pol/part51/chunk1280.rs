//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1280/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1280(t31101: f64, t81159: f64, t22716: f64, t8455: f64, t22704: f64, t31091: f64, t81326: f64, t2006: f64, t213: f64, t225: f64, t31138: f64, t6883: f64) -> (f64, f64, f64, f64, f64) {
    let t114255 = t81159 * t31101;
    let t114264 = 0.12793931631041761173e0_f64 * t22716 * t8455;
    let t114278 = t22704 * t81326 * t31091;
    let t114285 = t213 * t2006 * t225;
    let t114291 = t6883 * t31138;
    (t114255, t114264, t114278, t114285, t114291)
}
