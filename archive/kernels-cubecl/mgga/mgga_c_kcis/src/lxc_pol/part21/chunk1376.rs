//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1376/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1376<F: Float>(t96395: F, t96401: F, t26999: F, t27077: F, t28190: F, t7772: F, t7788: F, t7791: F, t93173: F, t93196: F, t96399: F, t96720: F, t97010: F, t97063: F, t97069: F, t97258: F) -> F {
    let t97442 = F::cast_from(0.10317654320987654321e-2_f64) * t96395;
    let t97449 = F::cast_from(0.15476481481481481481e-2_f64) * t96401;
    let t97454 = -F::cast_from(0.15476481481481481481e-2_f64) * t93173 - F::cast_from(0.69505208333333333334e-3_f64) * t28190 * t26999 - F::cast_from(0.46377350260416666667e-4_f64) * t7772 * t97063 + F::cast_from(0.92835860883789062501e-5_f64) * t27077 * t97069 + t97442 - F::cast_from(0.41270617283950617284e-2_f64) * t96399 + F::cast_from(0.7722800925925925926e-4_f64) * t93196 + F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t97258 + F::cast_from(0.61782407407407407408e-3_f64) * t97010 * t7791 - t97449 + F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t97069 + F::cast_from(0.557015165302734375e-4_f64) * t27077 * t96720;
    t97454
}
