//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1065/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1065(t898: f64, t9688: f64, t338: f64, t353: f64, t2246: f64, t3099: f64, t2118: f64, t8652: f64, t3074: f64, t3202: f64, t840: f64, t3306: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9689 = t898 * t9688;
    let t9691 = t338 * t353 * t9689;
    let t9695 = 7.0_f64 / 72.0_f64 * t2246 * t3099;
    let t9696 = t2118 * t8652;
    let t9697 = t3074 * t9696;
    let t9701 = 7.0_f64 / 144.0_f64 * t840 * t3202;
    let t9702 = t3306 * t810;
    (t9689, t9691, t9695, t9697, t9701, t9702)
}
