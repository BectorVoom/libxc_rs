//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1058/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1058(t127122: f64, t127124: f64, t127125: f64, t128464: f64, t128466: f64, t128474: f64, t128475: f64, t128477: f64, t128482: f64, t130377: f64, t130443: f64, t130444: f64, t1849: f64, t2036: f64, t2075: f64, t29201: f64, t29378: f64, t29493: f64, t29848: f64, t34146: f64, t510: f64, t574: f64, t7890: f64, t7983: f64, t8329: f64, t8690: f64) -> f64 {
    let t130455 = -t127122 - t127124 - t127125 - 2.0_f64 * t7983 * t7890 - 2.0_f64 * t29493 * t2075 + (t130443 + t130444) * t574 - t2036 * t29848 - 2.0_f64 * t8690 * t29201 - 2.0_f64 * t130377 * t510 + 2.0_f64 * t34146 * t1849 + t128464 + t128466 + t8690 * t29378 + t128474 - t128475 - t8329 - t128477 - t128482;
    t130455
}
