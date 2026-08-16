//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1204/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1204(t2242: f64, t4055: f64, t2306: f64, t332: f64, t2382: f64, t2419: f64, t859: f64, t4387: f64, t892: f64, t13928: f64, t4386: f64, t13911: f64, t19906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51572 = t2242 * t4055;
    let t51580 = t2306 * t332;
    let t51581 = t2382 * t51580;
    let t51584 = t859 * t2419;
    let t51588 = t859 * t892 * t4387;
    let t51592 = t4386 * t892 * t13928;
    let t51595 = t19906 * t13911;
    (t51572, t51581, t51584, t51588, t51592, t51595)
}
