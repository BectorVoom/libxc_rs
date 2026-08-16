//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1179/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1179(t2271: f64, t810: f64, t2079: f64, t3037: f64, t858: f64, t892: f64, t1114: f64, t20112: f64, t27691: f64, t328: f64, t2118: f64, t3074: f64) -> (f64, f64, f64, f64, f64) {
    let t29117 = t2271 * t810;
    let t29287 = t2079 * t3037;
    let t29751 = t858 * t892;
    let t29775 = t1114 * t20112;
    let t29843 = t27691 * t328;
    let t29845 = t3074 * t2118 * t29843;
    (t29117, t29287, t29751, t29775, t29845)
}
