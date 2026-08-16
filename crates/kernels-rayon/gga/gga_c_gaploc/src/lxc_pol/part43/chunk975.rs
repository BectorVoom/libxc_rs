//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 975/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk975(t13880: f64, t784: f64, t13884: f64, t2049: f64, t47311: f64, t739: f64, t531: f64, t797: f64, t13879: f64, t2009: f64, t773: f64, t38950: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47417 = 0.23833659967900284446e0_f64 * t13880 * t784;
    let t47419 = 0.35750489951850426669e0_f64 * t2049 * t13884;
    let t47420 = t739 * t47311;
    let t47423 = 0.35750489951850426669e0_f64 * t797 * t531 * t47420;
    let t47430 = 0.35750489951850426669e0_f64 * t773 * t13879 * t2009;
    let t47432 = t955 * t38950;
    (t47417, t47419, t47420, t47423, t47430, t47432)
}
