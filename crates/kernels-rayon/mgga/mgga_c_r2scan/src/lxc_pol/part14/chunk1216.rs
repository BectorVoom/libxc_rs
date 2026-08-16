//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1216/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1216(t37835: f64, t37838: f64, t37841: f64, t37843: f64, t37848: f64, t37851: f64, t38528: f64, t38532: f64, t39740: f64, t39742: f64, t39746: f64, t39749: f64) -> f64 {
    let t41537 = t38528 + t38532 + 0.11708928647259339623e0_f64 * t37835 + 0.90044238659382329742e0_f64 * t37838 + 0.27013271597814698923e1_f64 * t37841 - 0.17336443480108537126e0_f64 * t39740 - 0.86682217400542685632e-1_f64 * t39742 - 0.5200933044032561138e0_f64 * t39746 + 0.26198215989259945076e-1_f64 * t39749 + 0.54878743191129263322e-2_f64 * t37843 - 0.16951189180550569635e1_f64 * t37848 - 0.50853567541651708904e1_f64 * t37851;
    t41537
}
