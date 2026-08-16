//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1222/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1222(t43083: f64, t43086: f64, t43088: f64, t43090: f64, t43092: f64, t43094: f64, t43097: f64, t43100: f64, t43103: f64, t43105: f64, t43108: f64, t43111: f64) -> f64 {
    let t44229 = -0.20803732176130244552e1_f64 * t43083 + 0.86682217400542685632e-1_f64 * t43086 - 0.54878743191129263322e-1_f64 * t43088 - 0.10975748638225852664e0_f64 * t43090 + 0.10975748638225852664e0_f64 * t43092 + 0.17336443480108537126e0_f64 * t43094 - 0.95219938395347901947e-2_f64 * t43097 + 0.47609969197673950973e-2_f64 * t43100 - 0.28565981518604370584e-1_f64 * t43103 + 0.95219938395347901947e-2_f64 * t43105 - 0.10401866088065122276e1_f64 * t43108 + 0.47609969197673950973e-2_f64 * t43111;
    t44229
}
