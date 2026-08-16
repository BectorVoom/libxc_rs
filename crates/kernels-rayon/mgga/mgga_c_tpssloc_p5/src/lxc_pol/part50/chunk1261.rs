//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1261/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1261(t1814: f64, t31175: f64, t8467: f64, t26288: f64, t5308: f64, t6950: f64, t114012: f64, t114026: f64, t114028: f64, t114031: f64, t114034: f64, t114039: f64, t114046: f64, t120388: f64, t120393: f64, t120395: f64, t120397: f64, t120399: f64, t120401: f64, t120405: f64, t120408: f64, t120410: f64, t120413: f64) -> f64 {
    let t120416 = t1814 * t31175 * t8467;
    let t120419 = t26288 * t6950 * t5308;
    let t120424 = 0.16149102437656156342e-2_f64 * t120388 + 7.0_f64 / 2304.0_f64 * t114012 + 0.80745512188280781708e-3_f64 * t120393 + t120395 / 384.0_f64 - t120397 / 1536.0_f64 + t120399 / 384.0_f64 + t120401 / 768.0_f64 + t114026 - 0.48447307312968469025e-2_f64 * t120405 - 0.80745512188280781708e-3_f64 * t120408 + 0.56521858531796547196e-2_f64 * t120410 + t120413 / 1536.0_f64 - 7.0_f64 / 2304.0_f64 * t120416 + 0.33913115119077928318e-1_f64 * t120419 + t114028 + 0.80745512188280781708e-3_f64 * t114031 - 7.0_f64 / 2304.0_f64 * t114034 + t114039 + 0.13457585364713463618e-3_f64 * t114046;
    t120424
}
