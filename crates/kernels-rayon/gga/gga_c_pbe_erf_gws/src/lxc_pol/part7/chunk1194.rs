//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1194/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1194(t6241: f64, t810: f64, t20296: f64, t2170: f64, t8903: f64, t3138: f64, t6287: f64, t2168: f64, t6177: f64, t6523: f64, t6524: f64, t6238: f64, t837: f64, t863: f64) -> (f64, f64, f64, f64, f64) {
    let t21227 = t6241 * t810;
    let t21231 = t8903 * t2170 * t20296 * t21227 / 2.0_f64;
    let t21239 = t3138 * t2170 * t20296 * t6287 / 2.0_f64;
    let t21243 = 3.0_f64 / 8.0_f64 * t2168 * t6523 * t6177 * t6524;
    let t21245 = t863 * t6238 * t837;
    (t21227, t21231, t21239, t21243, t21245)
}
