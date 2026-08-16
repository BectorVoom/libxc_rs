//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 708/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk708(t1172: f64, t320: f64, t1198: f64, t2053: f64, t1105: f64, t3944: f64, t1123: f64, t3950: f64, t850: f64, t833: f64, t2409: f64, t3050: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4062 = t1172 * t320;
    let t4063 = t1198 * t2053;
    let t4123 = t3944 * t1105;
    let t4127 = t850 * t1123 * t3950;
    let t4128 = t4127 * t833;
    let t4130 = t2409 * t3050;
    (t4062, t4063, t4123, t4127, t4128, t4130)
}
