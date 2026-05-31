//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1233/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1233<F: Float>(t1048: F, t43006: F, t43023: F, t43056: F, t43064: F, t43081: F, t43113: F, t43143: F, t43171: F, t43193: F, t43211: F, t43227: F, t43244: F, t43264: F, t43279: F, t43310: F, t43342: F, t43374: F, t43403: F, t43423: F, t43443: F, t43457: F, t43485: F, t43493: F, t43523: F, t43557: F, t43583: F, t43601: F, t43627: F, t43652: F, t43674: F, t43687: F, t43707: F, t499: F, t797: F) -> F {
    let t43716 = t1048 * t499 * (t43310 + t43171 + t43674 + t43443 + t43244 + t43264 + t43211 + t43227 + t43557 + t43143 + t43493 + t43374 + t43523 + t43081 + t43423 + t43279 + t43056 + t43485 + t43403 + t43457 + t43707 + t43113 + t43193 + t43583 + t43687 + t43006 + t43652 + t43627 + t43342 + t43064 + t43601 + t43023) * t797 / F::cast_from(4.0_f64);
    t43716
}
