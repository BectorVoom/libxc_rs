//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1199/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1199(t13829: f64, t1646: f64, t528: f64, t13818: f64, t1599: f64, t46953: f64, t531: f64, t557: f64, t42230: f64, t42233: f64, t42236: f64, t42239: f64, t42242: f64, t42245: f64, t42250: f64, t42254: f64, t42257: f64) -> f64 {
    let t48093 = 0.35750489951850426669e0_f64 * t528 * t13829 * t1646;
    let t48096 = 0.35750489951850426669e0_f64 * t1599 * t13818;
    let t48099 = 0.35750489951850426669e0_f64 * t557 * t531 * t46953;
    let t48100 = -t48093 - t42230 + t42233 + t42236 + t42239 - t42242 + t42245 + t42250 + 0.42900587942220512003e1_f64 * t42254 + t42257 - t48096 - t48099;
    t48100
}
