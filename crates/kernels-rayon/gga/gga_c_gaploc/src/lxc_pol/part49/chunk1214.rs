//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1214/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1214(t224: f64, t42496: f64, t42501: f64, t42503: f64, t42506: f64, t42509: f64, t42513: f64, t47074: f64, t47075: f64, t47078: f64, t47089: f64, t47092: f64, t47095: f64, t47098: f64, t47108: f64, t47109: f64, t47110: f64, t47124: f64, t47789: f64, t48243: f64) -> f64 {
    let t48248 = t42496 - t47074 + t42501 + t42503 + t42506 - t47075 + t47078 + t42509 + t224 * (t47089 + t47092 + t47095 + t47098 + t47109 + t47124 + t47789 + t48243) + t47108 + t47110 - t42513;
    t48248
}
