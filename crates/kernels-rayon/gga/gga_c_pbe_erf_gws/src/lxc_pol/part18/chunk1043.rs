//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1043/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1043(t11693: f64, t8903: f64, t11459: f64, t3139: f64, t3140: f64, t3138: f64, t11618: f64, t254: f64, t906: f64, t369: f64, t3772: f64, t3848: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11695 = t8903 * t11693 / 16.0_f64;
    let t11697 = t3139 * t11459 * t3140;
    let t11699 = t3138 * t11697 / 48.0_f64;
    let t11700 = t254 * t11618;
    let t11701 = t11700 * t906;
    let t11706 = t3772 * t369;
    let t11717 = t3848 * t810;
    (t11695, t11697, t11699, t11701, t11706, t11717)
}
