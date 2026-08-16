//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1101/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1101<F: Float>(t30: F, t1468: F, t9335: F, t2: F, t3833: F, t580: F, t605: F, t22: F, t2257: F, t3834: F, t513: F, t5549: F, t5552: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t13550 = t9335 * t1468;
    let t13553 = t3833 * t2;
    let t13554 = t580 * t605;
    let t13564 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13550 * t3834 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13553 * t13554 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5549 * t2257 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t513 * t580 - F::cast_from(8.0_f64) * t5552 * t22);
    (t13554, t13564)
}
