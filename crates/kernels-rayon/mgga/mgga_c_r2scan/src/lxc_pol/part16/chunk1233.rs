//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1233/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1233(t1048: f64, t43006: f64, t43023: f64, t43056: f64, t43064: f64, t43081: f64, t43113: f64, t43143: f64, t43171: f64, t43193: f64, t43211: f64, t43227: f64, t43244: f64, t43264: f64, t43279: f64, t43310: f64, t43342: f64, t43374: f64, t43403: f64, t43423: f64, t43443: f64, t43457: f64, t43485: f64, t43493: f64, t43523: f64, t43557: f64, t43583: f64, t43601: f64, t43627: f64, t43652: f64, t43674: f64, t43687: f64, t43707: f64, t499: f64, t797: f64) -> f64 {
    let t43716 = t1048 * t499 * (t43310 + t43171 + t43674 + t43443 + t43244 + t43264 + t43211 + t43227 + t43557 + t43143 + t43493 + t43374 + t43523 + t43081 + t43423 + t43279 + t43056 + t43485 + t43403 + t43457 + t43707 + t43113 + t43193 + t43583 + t43687 + t43006 + t43652 + t43627 + t43342 + t43064 + t43601 + t43023) * t797 / 4.0_f64;
    t43716
}
