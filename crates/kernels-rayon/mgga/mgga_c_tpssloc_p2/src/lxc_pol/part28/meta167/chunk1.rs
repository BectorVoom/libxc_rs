//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 822/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk822(t500: f64, t1256: f64, t193: f64, t3258: f64, t3261: f64, t3268: f64, t3310: f64, t3318: f64, t336: f64, t3408: f64, t3410: f64, t3413: f64, t3417: f64, t3421: f64, t3425: f64, t3633: f64, t3637: f64) -> (f64, f64, f64) {
    let t3639 = t500 * t500;
    let t3640 = 1.0_f64 / t3639;
    let t3643 = t1256 * t193 * t336 * t3633 - t193 * t336 * t3637 * t3640 - t3258 + t3261 - t3268 + t3310 + t3318 + t3408 + t3410 - t3413 + t3417 - t3421 - t3425;
    (t3639, t3640, t3643)
}
