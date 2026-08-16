//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1011/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1011(t33358: f64, t91655: f64, t127107: f64, t127109: f64, t127111: f64, t128298: f64, t128300: f64, t128302: f64, t128303: f64, t128306: f64, t128371: f64, t128375: f64, t128377: f64, t128381: f64, t128383: f64, t128385: f64, t1849: f64, t31532: f64, t33601: f64, t510: f64, t5460: f64, t6287: f64, t8519: f64) -> f64 {
    let t128387 = 6.0_f64 * t91655 * t33358;
    let t128388 = -t128371 * t510 + 2.0_f64 * t1849 * t33601 - 4.0_f64 * t31532 * t5460 - t6287 * t8519 - t127107 - t127109 - t127111 - t128298 - t128300 - t128302 + t128303 - t128306 - t128375 - t128377 - t128381 - t128383 - t128385 - t128387;
    t128388
}
