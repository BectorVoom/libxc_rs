//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1068/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1068<F: Float>(t39420: F, t43026: F, t43029: F, t43032: F, t43034: F, t43037: F, t43040: F, t43042: F, t43045: F, t43048: F, t43051: F, t43054: F, t38452: F, t39429: F, t39464: F, t39470: F, t39482: F, t41384: F, t41385: F, t41386: F, t41387: F, t41392: F, t43057: F, t43061: F) -> (F, F) {
    let t44202 = -0.13869154784086829701e1 * t43026 - 0.86682217400542685632e-1 * t43029 - 0.51220160311720645767e0 * t39420 - 0.86682217400542685632e-1 * t43032 + 0.17336443480108537126e0 * t43034 + 0.17336443480108537126e0 * t43037 + 0.17336443480108537126e0 * t43040 + 0.5200933044032561138e0 * t43042 + 0.5200933044032561138e0 * t43045 + 0.5200933044032561138e0 * t43048 + 0.86682217400542685632e-1 * t43051 + 0.2600466522016280569e0 * t43054;
    let t44209 = 0.10975748638225852664e0 * t43057 + 0.62295486109113302474e-1 * t39429 + t41384 - t41385 - t41386 + t41387 - t41392 - 0.23804984598836975487e0 * t39464 - 0.57829097596741960691e-3 * t39470 + 0.87327386630866483588e-2 * t43061 - t38452 + 0.62295486109113302474e-1 * t39482;
    (t44202, t44209)
}
