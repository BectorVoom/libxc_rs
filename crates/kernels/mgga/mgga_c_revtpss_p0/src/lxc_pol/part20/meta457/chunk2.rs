//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1744/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1744<F: Float>(t30: F, t525: F, t9603: F, t2257: F, t3833: F, t3834: F, t39456: F, t46311: F, t46317: F, t513: F, t9335: F, t9339: F, t9344: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t47025 = F::new(1.0) / t525 / t9603;
    let t47038 = piecewise3::<F>(t31, F::new(0.0), F::new(40.0) / F::new(81.0) * t47025 * t46311 - F::new(16.0) / F::new(9.0) * t9335 * t3834 * t2257 + F::new(4.0) / F::new(3.0) * t3833 * t46317 + F::new(16.0) / F::new(9.0) * t9339 * t9344 + F::new(4.0) / F::new(3.0) * t513 * t39456);
    t47038
}
