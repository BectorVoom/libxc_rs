//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1085/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1085<F: Float>(t38183: F, t38190: F, t40258: F, t40261: F, t43688: F, t43690: F, t43692: F, t43695: F, t43697: F, t43700: F, t43702: F, t43705: F, t1048: F, t43006: F, t43023: F, t43056: F, t43064: F, t43081: F, t43113: F, t43143: F, t43171: F, t43193: F, t43211: F, t43227: F, t43244: F, t43264: F, t43279: F, t43310: F, t43342: F, t43374: F, t43403: F, t43423: F, t43443: F, t43457: F, t43485: F, t43493: F, t43523: F, t43557: F, t43583: F, t43601: F, t43627: F, t43652: F, t43674: F, t43687: F, t499: F, t797: F) -> (F,) {
    let t43707 = -0.54878743191129263322e-1 * t43688 + 0.86682217400542685632e-1 * t43690 + 0.29272321618148349057e-1 * t43692 - 0.16463622957338778997e-1 * t38183 + t38190 + t40258 - 0.12805040077930161442e0 * t43695 - 0.43341108700271342816e-1 * t43697 - 0.43341108700271342816e-1 * t43700 - 0.43341108700271342816e-1 * t43702 - t40261 + 0.65495539973149862688e-2 * t43705;
    let t43716 = t1048 * t499 * (t43310 + t43171 + t43674 + t43443 + t43244 + t43264 + t43211 + t43227 + t43557 + t43143 + t43493 + t43374 + t43523 + t43081 + t43423 + t43279 + t43056 + t43485 + t43403 + t43457 + t43707 + t43113 + t43193 + t43583 + t43687 + t43006 + t43652 + t43627 + t43342 + t43064 + t43601 + t43023) * t797 / 4.0;
    (t43716,)
}
