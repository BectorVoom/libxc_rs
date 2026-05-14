//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1215/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1215<F: Float>(t102197: F, t102205: F, t28369: F, t28485: F, t28489: F, t28495: F, t98119: F, t98365: F, t98381: F, t98383: F, t98387: F, t98388: F, t98390: F, t1307: F, t20916: F, t5709: F) -> (F, F) {
    let t103418 = -0.61836467013888888889e-4 * t98365 + 0.55273148148148148147e-3 * t102197 + t98381 + 0.73697530864197530862e-3 * t102205 - t98383 - t98387 + 0.12356481481481481482e-2 * t98388 + 0.30891203703703703704e-3 * t98390 + 0.46336805555555555556e-3 * t28369 * t28485 + 0.92673611111111111112e-3 * t28369 * t28489 + 0.61836467013888888889e-4 * t98119 * t28485 - 0.61782407407407407408e-3 * t28369 * t28495;
    let t103423 = t5709 * t20916 * t1307;
    (t103418, t103423)
}
