//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 712/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk712(t3550: f64, t401: f64, t3351: f64, t5002: f64, t5063: f64, t3466: f64, t395: f64, t3470: f64, t3474: f64, t3583: f64, t719: f64, t256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10521 = t401 * t3550;
    let t10523 = t5002 * t3351;
    let t10534 = t5063 * t3351;
    let t10581 = t395 * t3466;
    let t10583 = t395 * t3470;
    let t10585 = t395 * t3474;
    let t10606 = t3583 * t719;
    let t10607 = t10606 * t256;
    (t10521, t10523, t10534, t10581, t10583, t10585, t10606, t10607)
}
