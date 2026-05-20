//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3825/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3825<F: Float>(t30: F, t13687: F, t14: F, t18280: F, t21944: F, t21949: F, t2257: F, t27: F, t3834: F, t3874: F, t46310: F, t48394: F, t5574: F, t580: F, t5824: F, t605: F, t6785: F, t73423: F, t9342: F, t9605: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t73552 = piecewise3::<F>(t31, F::new(0.0), -F::new(56.0) / F::new(81.0) * t46310 * t6785 * t3834 + F::new(64.0) / F::new(27.0) * t13687 * t73423 + F::new(8.0) / F::new(27.0) * t21944 * t2257 - F::new(16.0) / F::new(9.0) * t3874 * t14 * t27 - F::new(8.0) / F::new(9.0) * t5574 * t580 + F::new(8.0) / F::new(3.0) * t5574 * t9342 + F::new(8.0) / F::new(27.0) * t9605 * t5824 * t3834 - F::new(4.0) / F::new(9.0) * t3874 * t18280 * t605 - F::new(2.0) / F::new(9.0) * t21949 * t2257 + t48394);
    t73552
}
