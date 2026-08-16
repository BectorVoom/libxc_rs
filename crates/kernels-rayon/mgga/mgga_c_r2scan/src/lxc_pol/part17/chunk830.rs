//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 830/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk830(t5109: f64, t8764: f64, t2122: f64, t2133: f64, t5101: f64, t5108: f64, t6132: f64, t6139: f64, t6293: f64, t6583: f64, t7235: f64, t7237: f64, t7259: f64, t7263: f64, t7298: f64, t7312: f64, t7317: f64, t8737: f64, t8742: f64, t8746: f64, t8749: f64, t8753: f64, t8757: f64, t8761: f64) -> f64 {
    let t8765 = t5109 * t8764;
    let t8768 = -0.25426783770825854452e1_f64 * t7235 - 0.85366933852867742947e0_f64 * t7237 - 0.12695991786046386925e-1_f64 * t7259 - 0.38087975358139160777e-1_f64 * t7263 + 0.16262400898971305031e-3_f64 * t7298 + t7312 + t7317 - 0.16463622957338778997e-1_f64 * t5101 + 0.86682217400542685632e-1_f64 * t2133 * t8737 - 0.21951497276451705328e0_f64 * t2122 * t8742 - 0.17336443480108537126e0_f64 * t6132 * t8746 - 0.5200933044032561138e0_f64 * t6139 * t8749 + 0.10975748638225852664e0_f64 * t2122 * t8753 - 0.32927245914677557992e0_f64 * t6293 * t8757 - 0.2600466522016280569e0_f64 * t5108 * t8761 - 0.17336443480108537126e0_f64 * t6583 * t8765;
    t8768
}
