//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1051/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1051(t116135: f64, t127107: f64, t128306: f64, t128375: f64, t128377: f64, t128381: f64, t128383: f64, t128385: f64, t129164: f64, t2039: f64, t2096: f64, t27188: f64, t29205: f64, t29211: f64, t29243: f64, t29247: f64, t29501: f64, t29848: f64, t32350: f64, t34150: f64, t5460: f64, t652: f64, t7042: f64, t7266: f64, t7458: f64, t7801: f64, t7989: f64, t8103: f64, t8690: f64) -> f64 {
    let t130326 = -2.0_f64 * t2039 * t29848 * t652 - 4.0_f64 * t652 * t7801 * t8103 - 6.0_f64 * t116135 * t29247 + t129164 * t2096 - 4.0_f64 * t27188 * t7989 - 4.0_f64 * t29205 * t7266 - 2.0_f64 * t29211 * t7266 + 2.0_f64 * t29243 * t8690 - 4.0_f64 * t29501 * t7042 - 4.0_f64 * t32350 * t5460 - 4.0_f64 * t34150 * t7458 - t127107 - t128306 - t128375 - t128377 - t128381 - t128383 - t128385;
    t130326
}
