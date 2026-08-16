//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 876/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk876(t173: f64, t7559: f64, t184: f64, t199: f64, t626: f64, t7483: f64, t2735: f64, t211: f64, t1046: f64, t1783: f64, t2723: f64, t582: f64) -> (f64, f64, f64, f64) {
    let t7560 = t173 * t7559;
    let t7561 = t7560 * t184;
    let t7563 = 2.0_f64 / 15.0_f64 * t7561 * t199;
    let t7564 = t7483 * t626;
    let t7565 = t2735 * t7564;
    let t7567 = 8.0_f64 / 45.0_f64 * t211 * t7565;
    let t7569 = 4.0_f64 / 15.0_f64 * t1783 * t1046;
    let t7570 = t582 * t2723;
    (t7563, t7567, t7569, t7570)
}
