//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 831/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk831<F: Float>(t5109: F, t8764: F, t2122: F, t2133: F, t5101: F, t5108: F, t6132: F, t6139: F, t6293: F, t6583: F, t7235: F, t7237: F, t7259: F, t7263: F, t7298: F, t7312: F, t7317: F, t8737: F, t8742: F, t8746: F, t8749: F, t8753: F, t8757: F, t8761: F) -> F {
    let t8765 = t5109 * t8764;
    let t8768 = -F::cast_from(0.25426783770825854452e1_f64) * t7235 - F::cast_from(0.85366933852867742947e0_f64) * t7237 - F::cast_from(0.12695991786046386925e-1_f64) * t7259 - F::cast_from(0.38087975358139160777e-1_f64) * t7263 + F::cast_from(0.16262400898971305031e-3_f64) * t7298 + t7312 + t7317 - F::cast_from(0.16463622957338778997e-1_f64) * t5101 + F::cast_from(0.86682217400542685632e-1_f64) * t2133 * t8737 - F::cast_from(0.21951497276451705328e0_f64) * t2122 * t8742 - F::cast_from(0.17336443480108537126e0_f64) * t6132 * t8746 - F::cast_from(0.5200933044032561138e0_f64) * t6139 * t8749 + F::cast_from(0.10975748638225852664e0_f64) * t2122 * t8753 - F::cast_from(0.32927245914677557992e0_f64) * t6293 * t8757 - F::cast_from(0.2600466522016280569e0_f64) * t5108 * t8761 - F::cast_from(0.17336443480108537126e0_f64) * t6583 * t8765;
    t8768
}
