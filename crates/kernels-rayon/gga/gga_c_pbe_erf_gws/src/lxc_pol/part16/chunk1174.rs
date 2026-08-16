//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1174/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1174(t2200: f64, t857: f64, t329: f64, t1114: f64, t19658: f64, t2409: f64, t3205: f64, t1105: f64, t814: f64, t2074: f64, t2501: f64, t3199: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22508 = t2200 * t857;
    let t22509 = t329 * t22508;
    let t26604 = t1114 * t19658;
    let t26617 = t3205 * t2409;
    let t26623 = t1105 * t814;
    let t26647 = t2501 * t2074;
    let t26654 = t3199 * t898;
    (t22509, t26604, t26617, t26623, t26647, t26654)
}
