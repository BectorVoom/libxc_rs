//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3288/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3288(t5599: f64, t689: f64, t6919: f64, t5741: f64, t74892: f64, t22315: f64, t48084: f64, t22858: f64, t47372: f64, t686: f64, t72: f64, t1432: f64, t22964: f64) -> (f64, f64, f64, f64, f64) {
    let t86346 = t689 * t5599 * t6919;
    let t86350 = t74892 * t5741;
    let t86354 = t48084 * t22315;
    let t86358 = t47372 * t22858 * t72 * t686;
    let t86374 = t1432 * t22964 * t72 * t686;
    (t86346, t86350, t86354, t86358, t86374)
}
