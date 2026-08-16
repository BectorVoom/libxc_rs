//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 769/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk769(t1354: f64, t19: f64, t336: f64, t714: f64, t1627: f64, t1631: f64, t155: f64, t641: f64, t644: f64, t639: f64, t1782: f64, t586: f64) -> (f64, f64, f64, f64, f64) {
    let t5450 = t1354 * t19;
    let t5451 = t5450 * t336;
    let t5452 = t5451 * t714;
    let t5459 = t1627 * t1631;
    let t5463 = t155 * t641;
    let t5464 = t5463 * t644;
    let t5465 = t639 * t5464;
    let t5467 = t1782 * t586;
    (t5452, t5459, t5463, t5465, t5467)
}
