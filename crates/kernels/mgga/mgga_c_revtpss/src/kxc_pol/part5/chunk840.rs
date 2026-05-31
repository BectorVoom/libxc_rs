//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 840/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk840<F: Float>(t45: F, t57: F, t4399: F, t5819: F, t5825: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t5948 = F::cast_from(0.11696447245269292414e1_f64) * t4399;
    let t5954 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t80 * t5819 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t5825);
    let t5960 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t83 * t5819 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t5825);
    let t5962 = t5954 / F::cast_from(2.0_f64) + t5960 / F::cast_from(2.0_f64);
    (t5948, t5962)
}
