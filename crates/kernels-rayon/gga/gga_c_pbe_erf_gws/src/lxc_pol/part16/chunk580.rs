//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 580/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk580(t34: f64, t726: f64, t93: f64, t954: f64, t728: f64, t108: f64, t2538: f64, t418: f64, t422: f64, t532: f64, t1764: f64, t950: f64) -> (f64, f64, f64, f64, f64) {
    let t2541 = t726 * t34;
    let t2544 = t93 * t954;
    let t2547 = t728 * t34;
    let t2551 = (20.0_f64 / 9.0_f64 * t2538 * t418 + 8.0_f64 / 3.0_f64 * t2541 * t532 + 20.0_f64 / 9.0_f64 * t2544 * t422 - 8.0_f64 / 3.0_f64 * t2547 * t532) * t108;
    let t2554 = t1764 * t950;
    (t2541, t2544, t2547, t2551, t2554)
}
