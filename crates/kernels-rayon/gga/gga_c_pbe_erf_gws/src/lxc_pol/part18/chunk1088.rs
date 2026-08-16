//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1088/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1088(t12149: f64, t12150: f64, t12152: f64, t12153: f64, t12156: f64, t12157: f64, t12159: f64, t12160: f64, t339: f64, t338: f64, t376: f64, t9807: f64) -> (f64, f64, f64) {
    let t12163 = t12149 + t12150 + t12152 + t12153 + t12156 + t12157 + t12159 + t12160;
    let t12164 = t339 * t12163;
    let t12166 = t338 * t12164 * t376;
    let t12169 = t376 * t9807;
    (t12164, t12166, t12169)
}
