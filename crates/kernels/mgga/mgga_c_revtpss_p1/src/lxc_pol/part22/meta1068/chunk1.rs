//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3820/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3820<F: Float>(t30: F, t47060: F, t2: F, t580: F, t605: F, t13550: F, t14: F, t18280: F, t21906: F, t21911: F, t2257: F, t27: F, t3833: F, t3834: F, t47025: F, t48185: F, t5549: F, t5824: F, t6785: F, t9335: F, t9342: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t73418 = F::cast_from(0.11696447245269292414e1_f64) * t47060;
    let t73423 = t605 * t2 * t580;
    let t73444 = piecewise3::<F>(t31, F::new(0.0), F::new(40.0) / F::new(81.0) * t47025 * t6785 * t3834 - F::new(64.0) / F::new(27.0) * t13550 * t73423 - F::new(8.0) / F::new(27.0) * t21906 * t2257 + F::new(32.0) / F::new(9.0) * t3833 * t14 * t27 + F::new(16.0) / F::new(9.0) * t5549 * t580 - F::new(16.0) / F::new(3.0) * t5549 * t9342 - F::new(8.0) / F::new(27.0) * t9335 * t5824 * t3834 + F::new(8.0) / F::new(9.0) * t3833 * t18280 * t605 + F::new(4.0) / F::new(9.0) * t21911 * t2257 + t48185);
    (t73418, t73423, t73444)
}
