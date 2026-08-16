//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1199/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1199(t13920: f64, t51563: f64, t2306: f64, t332: f64, t2382: f64, t2419: f64, t859: f64, t4387: f64, t892: f64, t1477: f64, t326: f64, t886: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51564 = t51563 * t13920;
    let t51580 = t2306 * t332;
    let t51581 = t2382 * t51580;
    let t51584 = t859 * t2419;
    let t51588 = t859 * t892 * t4387;
    let t51649 = t326 * t1477;
    let t51650 = t51649 * t886;
    (t51564, t51581, t51584, t51588, t51649, t51650)
}
