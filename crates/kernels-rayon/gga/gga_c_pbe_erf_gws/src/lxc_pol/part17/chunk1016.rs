//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1016/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1016(t2170: f64, t2190: f64, t3178: f64, t2168: f64, t6606: f64, t6597: f64, t9110: f64, t9113: f64, t9114: f64, t9118: f64, t9121: f64, t9123: f64, t9124: f64, t9129: f64, t9133: f64) -> (f64, f64, f64, f64) {
    let t9135 = t2170 * t3178 * t2190;
    let t9137 = t2168 * t9135 / 48.0_f64;
    let t9138 = 7.0_f64 / 288.0_f64 * t6606;
    let t9139 = -t9110 - t9113 - t9114 - t9118 + t9121 + t9123 - t6597 - t9124 + t9129 + t9133 + t9137 + t9138;
    (t9135, t9137, t9138, t9139)
}
