//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 937/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk937(t8675: f64, t8686: f64, t8672: f64, t1647: f64, t1651: f64, t2258: f64, t2265: f64, t2266: f64, t2282: f64, t2294: f64, t37357: f64, t379: f64, t39439: f64, t39441: f64, t39449: f64, t39451: f64, t39453: f64, t39455: f64, t631: f64, t8634: f64, t8680: f64, t8709: f64) -> f64 {
    let t39457 = t8675 * t8686;
    let t39471 = t8675 * t8672;
    let t39481 = 10.0_f64 / 27.0_f64 * t39439 - 4.0_f64 / 9.0_f64 * t39441 + 2.0_f64 * t631 * t2258 * t8634 * t37357 - 40.0_f64 / 9.0_f64 * t39449 + 8.0_f64 / 3.0_f64 * t39451 + 8.0_f64 / 3.0_f64 * t39453 - 4.0_f64 / 9.0_f64 * t39455 - 16.0_f64 / 3.0_f64 * t39457 - 12.0_f64 * t2265 * t8680 * t1647 * t2282 - 4.0_f64 / 3.0_f64 * t2265 * t2266 * t379 * t8709 - 2.0_f64 * t2265 * t2266 * t1651 * t2294 + 8.0_f64 / 3.0_f64 * t39471 + 4.0_f64 * t2265 * t2266 * t1647 * t2294 + 6.0_f64 * t2265 * t8680 * t1651 * t2282;
    t39481
}
