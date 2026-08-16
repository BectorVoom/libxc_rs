//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 534/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk534(t1809: f64, t2673: f64, t639: f64, t1640: f64, t219: f64, t1642: f64, t954: f64, t422: f64) -> (f64, f64, f64, f64, f64) {
    let t2674 = t1809 * t2673;
    let t2676 = 8.0_f64 / 45.0_f64 * t639 * t2674;
    let t2677 = t1640 * t219;
    let t2678 = t1642 * t954;
    let t2679 = t2678 * t422;
    (t2674, t2676, t2677, t2678, t2679)
}
