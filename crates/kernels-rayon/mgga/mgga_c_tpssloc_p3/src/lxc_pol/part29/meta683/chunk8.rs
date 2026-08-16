//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2322/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2322(t27700: f64, t86261: f64, t15239: f64, t15474: f64, t15541: f64, t15761: f64, t24741: f64, t27617: f64, t3580: f64, t3587: f64, t475: f64, t68: f64, t7326: f64, t7328: f64, t7331: f64, t7345: f64, t86313: f64, t95545: f64, t95550: f64, t95556: f64, t95566: f64, t95571: f64) -> f64 {
    let t95573 = 0.20186378047070195428e-3_f64 * t86261 * t27700;
    let t95576 = t95545 + 5.0_f64 / 6912.0_f64 * t27617 * t3587 - t7345 * t15761 / 2304.0_f64 + t95550 / 10368.0_f64 - t24741 * t15474 / 2304.0_f64 + 0.20186378047070195428e-3_f64 * t86313 - 0.20186378047070195428e-3_f64 * t95556 * t7331 + 0.10093189023535097714e-3_f64 * t7326 * t7328 * t15239 * t68 * t475 + t95566 * t3580 / 216.0_f64 + t95571 - t95573 + 5.0_f64 / 3456.0_f64 * t7345 * t15541;
    t95576
}
