//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 942/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk942(t3923: f64, t3928: f64, t113: f64, t1266: f64, t1271: f64, t1393: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t3652: f64, t3660: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64) -> (f64, f64) {
    let t3929 = t3923 + t3928;
    let t3931 = -t113 * t3652 - 2.0_f64 * t1266 * t650 + 2.0_f64 * t1271 * t1393 - t2312 * t510 - 4.0_f64 * t2314 * t672 - 2.0_f64 * t2320 * t510 - 4.0_f64 * t2323 * t652 - 2.0_f64 * t2364 * t652 + t3660 * t574 + t3929 * t513;
    (t3929, t3931)
}
