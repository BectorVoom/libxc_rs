//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 524/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk524(t1628: f64, t3185: f64, t1532: f64, t1562: f64, t1580: f64, t1599: f64, t1641: f64, t193: f64, t3166: f64, t3169: f64, t3182: f64, t3186: f64, t4950: f64, t557: f64, t574: f64, t597: f64, t9484: f64, t9487: f64, t9490: f64, t9494: f64, t9497: f64, t9500: f64, t9503: f64) -> f64 {
    let t9506 = t1628 * t3185;
    let t9509 = 0.11502877786176224903e2_f64 * t1580 * t3182 - 0.23005755572352449806e1_f64 * t1641 * t3186 + 0.71500979903700853338e0_f64 * t4950 * t3166 - 0.35750489951850426669e0_f64 * t1599 * t3169 - 0.35750489951850426669e0_f64 * t557 * t9484 + 0.35750489951850426669e0_f64 * t9487 * t193 + 0.35750489951850426669e0_f64 * t9490 * t193 + 0.35750489951850426669e0_f64 * t9494 * t193 - 0.10725146985555128001e1_f64 * t9497 * t1532 - 0.92023022289409799224e1_f64 * t1562 * t9500 + 0.30674340763136599741e1_f64 * t597 * t9503 - 0.30674340763136599741e1_f64 * t574 * t9506;
    t9509
}
