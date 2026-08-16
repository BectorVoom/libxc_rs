//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1173/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1173(t22334: f64, t2306: f64, t3074: f64, t3039: f64, t4384: f64, t6792: f64, t2395: f64, t2494: f64, t1105: f64, t4417: f64, t1114: f64, t19776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22336 = t3074 * t2306 * t22334;
    let t22343 = t3039 * t4384;
    let t22379 = t3039 * t6792;
    let t22393 = t2395 * t2494;
    let t22410 = t4417 * t1105;
    let t22493 = t1114 * t19776;
    (t22336, t22343, t22379, t22393, t22410, t22493)
}
