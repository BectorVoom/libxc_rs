//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 815/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk815(t3205: f64, t336: f64, t2182: f64, t343: f64, t2122: f64, t337: f64, t810: f64, t2147: f64, t2133: f64, t2387: f64, t2153: f64, t837: f64, t863: f64) -> (f64, f64, f64, f64, f64) {
    let t6523 = t3205 * t336;
    let t6524 = t343 * t2182;
    let t6534 = t337 * t2122 * t810;
    let t6535 = t2147 * t6534;
    let t6538 = t2387 * t2133;
    let t6542 = t863 * t2153 * t837;
    (t6523, t6524, t6535, t6538, t6542)
}
