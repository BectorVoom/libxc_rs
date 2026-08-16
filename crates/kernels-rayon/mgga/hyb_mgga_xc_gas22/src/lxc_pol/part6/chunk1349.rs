//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1349/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1349(t10652: f64, t6574: f64, t24926: f64, t29079: f64, t29081: f64, t29384: f64, t29387: f64, t29392: f64, t29394: f64, t29396: f64, t29398: f64, t29400: f64, t29402: f64, t29404: f64, t29406: f64, t29408: f64, t29411: f64, t29414: f64, t29418: f64, t29420: f64, t8785: f64) -> (f64, f64) {
    let t29422 = 0.32163958997385070134e2_f64 * t6574 * t10652;
    let t29423 = t29079 + t29081 - t29384 + t29387 + 24.0_f64 * t24926 * t8785 + t29392 - t29394 + t29396 + t29398 - t29400 + t29402 - t29404 - t29406 + t29408 - t29411 - t29414 - t29418 + t29420 - t29422;
    (t29422, t29423)
}
