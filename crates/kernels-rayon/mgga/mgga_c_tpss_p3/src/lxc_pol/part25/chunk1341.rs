//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1341/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1341(t42667: f64, t5784: f64, t1792: f64, t18666: f64, t19388: f64, t19396: f64, t20246: f64, t5489: f64, t6304: f64, t67429: f64, t67431: f64, t67433: f64, t67436: f64, t67440: f64, t67451: f64, t67454: f64, t69097: f64, t69165: f64) -> f64 {
    let t71490 = t42667 * t5784;
    let t71499 = 10.0_f64 * t18666 * t69097 - 5.0_f64 / 3.0_f64 * t71490 * t5489 + t67429 + t67431 + t67433 + t67436 + t67440 - t67451 - t67454 - 2.0_f64 / 3.0_f64 * t69165 * t1792 - 10.0_f64 / 3.0_f64 * t20246 * t19388 - 4.0_f64 / 3.0_f64 * t19396 * t6304;
    t71499
}
