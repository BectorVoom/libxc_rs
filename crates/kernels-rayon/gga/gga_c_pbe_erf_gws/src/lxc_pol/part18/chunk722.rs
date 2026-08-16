//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 722/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk722(t4043: f64, t918: f64, t1189: f64, t925: f64, t366: f64, t864: f64, t899: f64) -> (f64, f64, f64) {
    let t4044 = t4043 * t918;
    let t4046 = t1189 * t925;
    let t4047 = 7.0_f64 / 2304.0_f64 * t4046;
    let t4049 = t899 * t864 * t366;
    (t4044, t4047, t4049)
}
