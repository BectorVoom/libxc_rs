//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1219/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1219(t39420: f64, t43026: f64, t43029: f64, t43032: f64, t43034: f64, t43037: f64, t43040: f64, t43042: f64, t43045: f64, t43048: f64, t43051: f64, t43054: f64) -> f64 {
    let t44202 = -0.13869154784086829701e1_f64 * t43026 - 0.86682217400542685632e-1_f64 * t43029 - 0.51220160311720645767e0_f64 * t39420 - 0.86682217400542685632e-1_f64 * t43032 + 0.17336443480108537126e0_f64 * t43034 + 0.17336443480108537126e0_f64 * t43037 + 0.17336443480108537126e0_f64 * t43040 + 0.5200933044032561138e0_f64 * t43042 + 0.5200933044032561138e0_f64 * t43045 + 0.5200933044032561138e0_f64 * t43048 + 0.86682217400542685632e-1_f64 * t43051 + 0.2600466522016280569e0_f64 * t43054;
    t44202
}
