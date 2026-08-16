//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 782/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk782(t671: f64, t6867: f64, t6869: f64, t6871: f64, t7264: f64, t7266: f64, t113: f64, t1266: f64, t1393: f64, t2114: f64, t2165: f64, t2167: f64, t510: f64, t574: f64, t650: f64, t652: f64, t6522: f64, t6524: f64, t6527: f64, t6537: f64, t672: f64, t6877: f64, t6882: f64, t6998: f64, t7001: f64, t7271: f64, t7408: f64) -> (f64, f64) {
    let t7412 = 2.0_f64 * t671 * t7266 + t6867 + t6869 + t6871 + t7264;
    let t7415 = -t113 * t7408 - t1266 * t2114 + t1393 * t2167 - t2165 * t650 - t510 * t7264 + t574 * t7412 - 2.0_f64 * t652 * t7271 - 2.0_f64 * t672 * t7266 - t6522 - t6524 - t6527 - t6537 + t6877 + t6882 + t6998 - t7001;
    (t7412, t7415)
}
