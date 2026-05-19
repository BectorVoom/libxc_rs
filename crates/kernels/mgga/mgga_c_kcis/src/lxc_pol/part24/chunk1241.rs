//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1241/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1241<F: Float>(t100162: F, t100170: F, t100229: F, t100235: F, t100244: F, t100257: F, t27014: F, t27028: F, t27077: F, t28132: F, t28179: F, t28190: F, t28204: F, t29161: F, t5329: F, t68040: F, t68045: F, t7788: F, t92787: F, t93050: F) -> F {
    let t100262 = -F::cast_from(0.13901041666666666667e-2_f64) * t28190 * t28179 - F::cast_from(0.25794135802469135802e-3_f64) * t100229 - F::cast_from(0.69505208333333333334e-3_f64) * t27014 * t29161 - F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t100235 + F::cast_from(0.208515625e-2_f64) * t7788 * t5329 * t92787 * t68045 + F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t100244 - F::cast_from(0.13901041666666666667e-2_f64) * t7788 * t5329 * t27028 * t68040 - F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t100170 - F::cast_from(0.92754700520833333334e-4_f64) * t28204 * t28132 - F::cast_from(0.185671721767578125e-4_f64) * t27077 * t100257 + F::cast_from(0.24777891269883300782e-5_f64) * t93050 * t100162;
    t100262
}
