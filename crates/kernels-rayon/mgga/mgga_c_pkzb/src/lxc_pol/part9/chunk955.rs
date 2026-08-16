//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 955/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk955(t7386: f64, t7389: f64, t5525: f64, t5543: f64, t5558: f64, t5560: f64, t5563: f64, t5566: f64, t7357: f64, t7393: f64, t7397: f64, t7401: f64) -> f64 {
    let t7465 = 0.33114e0_f64 * t7386;
    let t7466 = 0.33114e0_f64 * t7389;
    let t7473 = -0.301925e0_f64 * t5525 + 0.40256666666666666667e0_f64 * t7357 - t7465 - t7466 + 0.248355e0_f64 * t7393 + 0.49671e0_f64 * t7397 + 0.248355e0_f64 * t7401 - t5543 - t5558 + 0.5519e0_f64 * t5560 - 0.16557e0_f64 * t5563 - 0.16557e0_f64 * t5566;
    t7473
}
