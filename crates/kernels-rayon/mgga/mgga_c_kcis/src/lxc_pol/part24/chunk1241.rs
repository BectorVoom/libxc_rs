//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1241/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1241(t100162: f64, t100170: f64, t100229: f64, t100235: f64, t100244: f64, t100257: f64, t27014: f64, t27028: f64, t27077: f64, t28132: f64, t28179: f64, t28190: f64, t28204: f64, t29161: f64, t5329: f64, t68040: f64, t68045: f64, t7788: f64, t92787: f64, t93050: f64) -> f64 {
    let t100262 = -0.13901041666666666667e-2_f64 * t28190 * t28179 - 0.25794135802469135802e-3_f64 * t100229 - 0.69505208333333333334e-3_f64 * t27014 * t29161 - 0.69505208333333333334e-3_f64 * t7788 * t100235 + 0.208515625e-2_f64 * t7788 * t5329 * t92787 * t68045 + 0.69505208333333333334e-3_f64 * t7788 * t100244 - 0.13901041666666666667e-2_f64 * t7788 * t5329 * t27028 * t68040 - 0.69505208333333333334e-3_f64 * t7788 * t100170 - 0.92754700520833333334e-4_f64 * t28204 * t28132 - 0.185671721767578125e-4_f64 * t27077 * t100257 + 0.24777891269883300782e-5_f64 * t93050 * t100162;
    t100262
}
