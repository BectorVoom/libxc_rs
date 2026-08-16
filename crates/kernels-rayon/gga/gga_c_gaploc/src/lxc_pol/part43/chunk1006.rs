//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1006/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1006(t1: f64, t47008: f64, t1415: f64, t2413: f64, t13829: f64, t1646: f64, t528: f64, t13818: f64, t1599: f64, t46953: f64, t531: f64, t557: f64) -> (f64, f64, f64, f64, f64) {
    let t48086 = t47008 * t1;
    let t48087 = t1415 * t48086;
    let t48088 = t48087 * t2413;
    let t48093 = 0.35750489951850426669e0_f64 * t528 * t13829 * t1646;
    let t48096 = 0.35750489951850426669e0_f64 * t1599 * t13818;
    let t48099 = 0.35750489951850426669e0_f64 * t557 * t531 * t46953;
    (t48086, t48088, t48093, t48096, t48099)
}
