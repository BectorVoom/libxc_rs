//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2615/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2615<F: Float>(t30: F, t3834: F, t580: F, t2257: F, t605: F, t22: F, t5552: F, t588: F, t13550: F, t13553: F, t1468: F, t2: F, t3833: F, t47025: F, t513: F, t5549: F, t9335: F, t9336: F, t9344: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t48165 = t580 * t3834;
    let t48168 = t605 * t2257;
    let t48174 = t22 * t605;
    let t48177 = t580 * t2257;
    let t48185 = F::new(32.0) * t5552 * t588;
    let t48187 = piecewise3::<F>(t31, F::new(0.0), F::new(40.0) / F::new(81.0) * t47025 * t1468 * t9336 - F::new(16.0) / F::new(9.0) * t9335 * t2 * t48165 - F::new(8.0) / F::new(9.0) * t13550 * t48168 + F::new(8.0) / F::new(3.0) * t3833 * t580 * t605 - F::new(8.0) * t13553 * t48174 + F::new(8.0) / F::new(3.0) * t13553 * t48177 + F::new(4.0) / F::new(9.0) * t5549 * t9344 - F::new(16.0) * t513 * t22 + t48185);
    (t48165, t48168, t48174, t48177, t48187)
}
