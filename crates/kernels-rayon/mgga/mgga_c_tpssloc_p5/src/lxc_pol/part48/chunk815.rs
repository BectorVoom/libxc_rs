//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 815/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk815(t113: f64, t1266: f64, t2165: f64, t2167: f64, t22460: f64, t22467: f64, t22482: f64, t22563: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t24543: f64, t24545: f64, t24552: f64, t24924: f64, t24932: f64, t24935: f64, t24939: f64, t3929: f64, t510: f64, t574: f64, t650: f64, t652: f64, t672: f64, t7264: f64, t7266: f64, t7271: f64, t7408: f64) -> f64 {
    let t24949 = -t113 * t24924 - 2.0_f64 * t1266 * t7264 - t2165 * t2312 - 2.0_f64 * t2165 * t2320 + t2167 * t3929 - 4.0_f64 * t2314 * t7271 - 4.0_f64 * t2323 * t7266 - 2.0_f64 * t2364 * t7266 - t24543 * t510 - 4.0_f64 * t24545 * t652 - 2.0_f64 * t24552 * t652 - 4.0_f64 * t24932 * t672 - 2.0_f64 * t24935 * t510 + t24939 * t574 - 2.0_f64 * t650 * t7408 - t22460 - t22467 - t22482 - t22563;
    t24949
}
