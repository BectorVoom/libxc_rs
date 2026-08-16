//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 277/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk277(t376: f64, t810: f64, t353: f64, t338: f64, t326: f64, param_a_c: f64) -> (f64, f64, f64, f64) {
    let t845 = t376 * t810;
    let t846 = t353 * t845;
    let t847 = t338 * t846;
    let t850 = t326 * param_a_c;
    (t845, t846, t847, t850)
}
