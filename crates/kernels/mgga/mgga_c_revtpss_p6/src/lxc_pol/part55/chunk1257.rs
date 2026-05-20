//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1257/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1257<F: Float>(t128709: F, t7063: F, t7286: F, t27888: F, t32729: F, t121234: F, t121235: F, t122464: F, t122466: F, t122468: F, t122475: F, t122477: F, t122480: F, t125855: F) -> F {
    let t128802 = t7063 * t128709 * t7286;
    let t128806 = t32729 * t27888;
    let t128810 = F::cast_from(0.25389723392137995738e-1_f64) * t122464 - F::cast_from(0.14279934416275588154e-1_f64) * t122466 - t122468 - F::cast_from(0.25702851531048074406e-1_f64) * t128802 + F::cast_from(0.42839803248826764462e-1_f64) * t122475 - F::cast_from(0.76169170176413987214e-1_f64) * t122477 - t122480 + F::cast_from(0.14456046980341999104e-1_f64) * t128806 + t121234 + F::cast_from(0.37645955677973955999e-4_f64) * t121235 + F::cast_from(0.14874931683620404328e-2_f64) * t125855;
    t128810
}
