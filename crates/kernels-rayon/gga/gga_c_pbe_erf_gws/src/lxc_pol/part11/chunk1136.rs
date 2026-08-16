//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1136/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1136(t41385: f64, t41388: f64, t41395: f64, t41398: f64, t41401: f64, t41404: f64, t48092: f64, t48095: f64, t48099: f64, t48101: f64, t48102: f64, t48103: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48104 = 64.0_f64 / 45.0_f64 * t41385;
    let t48105 = 128.0_f64 / 45.0_f64 * t41388;
    let t48106 = 128.0_f64 / 45.0_f64 * t41395;
    let t48107 = 32.0_f64 / 15.0_f64 * t41398;
    let t48108 = 16.0_f64 / 45.0_f64 * t41401;
    let t48109 = 64.0_f64 / 27.0_f64 * t41404;
    let t48110 = t48092 - t48095 - t48099 - t48101 - t48102 + t48103 - t48104 - t48105 - t48106 + t48107 - t48108 - t48109;
    (t48104, t48105, t48106, t48107, t48108, t48109, t48110)
}
