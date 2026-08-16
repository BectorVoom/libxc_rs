//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 874/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk874(t3070: f64, t9270: f64, t1146: f64, t2242: f64, t353: f64, t858: f64) -> (f64, f64, f64) {
    let t9272 = 7.0_f64 / 72.0_f64 * t9270 * t3070;
    let t9275 = t2242 * t1146;
    let t9283 = t858 * t353;
    (t9272, t9275, t9283)
}
