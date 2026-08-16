//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1033/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1033(t11571: f64, t3258: f64, t2255: f64, t2157: f64, t3165: f64, t3219: f64, t3235: f64, t2319: f64, t3863: f64, t3703: f64, t5: f64, t6523: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11572 = t3258 * t11571;
    let t11573 = t2255 * t11572;
    let t11576 = t2157 * t3165;
    let t11578 = t3235 * t3219 * t11576;
    let t11581 = t2319 * t3863;
    let t11583 = t5 * t3703;
    let t11585 = t6523 * t11583 * t875;
    (t11572, t11573, t11576, t11578, t11581, t11583, t11585)
}
