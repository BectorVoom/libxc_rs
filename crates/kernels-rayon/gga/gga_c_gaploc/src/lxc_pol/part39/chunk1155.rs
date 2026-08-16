//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1155/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1155(t42999: f64, t43003: f64, t43006: f64, t43010: f64, t43014: f64, t43017: f64, t43019: f64, t43023: f64, t43028: f64, t43032: f64, t43035: f64, t13945: f64, t681: f64) -> (f64, f64) {
    let t47625 = 0.20508069947045931423e-1_f64 * t42999 + 0.15381052460284448567e-1_f64 * t43003 + t43006 - 0.17090058289204942852e-2_f64 * t43010 - t43014 - t43017 + t43019 - t43023 + t43028 + t43032 - 0.85450291446024714263e-3_f64 * t43035;
    let t47629 = 0.76905262301422242837e-2_f64 * t681 * t13945;
    (t47625, t47629)
}
