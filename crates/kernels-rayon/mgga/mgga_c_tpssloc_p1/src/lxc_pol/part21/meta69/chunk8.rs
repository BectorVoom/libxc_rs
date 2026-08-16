//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 511/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk511(t113: f64, t1266: f64, t1271: f64, t1393: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64) -> f64 {
    let t1395 = -t113 * t1266 + t1271 * t574 + t1393 * t513 - t510 * t650 - 2.0_f64 * t652 * t672;
    t1395
}
