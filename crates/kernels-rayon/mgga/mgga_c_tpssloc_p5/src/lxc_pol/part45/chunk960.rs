//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 960/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk960(t31124: f64, t6883: f64, t31101: f64, t81159: f64, t22635: f64, t26331: f64, t31099: f64, t3734: f64, t22716: f64, t8455: f64, t1985: f64, t214: f64, t225: f64, t22870: f64, t567: f64) -> (f64, f64, f64, f64, f64) {
    let t114253 = t6883 * t31124;
    let t114254 = 0.76763589786250567036e-1_f64 * t114253;
    let t114255 = t81159 * t31101;
    let t114256 = 0.15352717957250113407e0_f64 * t114255;
    let t114262 = 0.9869604401089358619e-1_f64 * t26331 * t22635 * t31099 * t3734;
    let t114264 = 0.12793931631041761173e0_f64 * t22716 * t8455;
    let t114270 = 0.16449340668482264365e-1_f64 * t1985 * t214 * t22870 * t225 * t567;
    (t114254, t114256, t114262, t114264, t114270)
}
