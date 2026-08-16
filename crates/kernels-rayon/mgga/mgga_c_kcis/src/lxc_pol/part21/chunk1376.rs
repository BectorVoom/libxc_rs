//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1376/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1376(t96395: f64, t96401: f64, t26999: f64, t27077: f64, t28190: f64, t7772: f64, t7788: f64, t7791: f64, t93173: f64, t93196: f64, t96399: f64, t96720: f64, t97010: f64, t97063: f64, t97069: f64, t97258: f64) -> f64 {
    let t97442 = 0.10317654320987654321e-2_f64 * t96395;
    let t97449 = 0.15476481481481481481e-2_f64 * t96401;
    let t97454 = -0.15476481481481481481e-2_f64 * t93173 - 0.69505208333333333334e-3_f64 * t28190 * t26999 - 0.46377350260416666667e-4_f64 * t7772 * t97063 + 0.92835860883789062501e-5_f64 * t27077 * t97069 + t97442 - 0.41270617283950617284e-2_f64 * t96399 + 0.7722800925925925926e-4_f64 * t93196 + 0.69505208333333333334e-3_f64 * t7788 * t97258 + 0.61782407407407407408e-3_f64 * t97010 * t7791 - t97449 + 0.69505208333333333334e-3_f64 * t7788 * t97069 + 0.557015165302734375e-4_f64 * t27077 * t96720;
    t97454
}
