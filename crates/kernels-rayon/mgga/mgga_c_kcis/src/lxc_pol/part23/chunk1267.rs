//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1267/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1267(t1380: f64, t27403: f64, t27455: f64, t27459: f64, t28372: f64, t28373: f64, t28375: f64, t28392: f64, t28480: f64, t4007: f64, t7908: f64, t7916: f64, t8151: f64, t94626: f64, t98205: f64, t98322: f64, t98673: f64, t98676: f64, t98680: f64, t98684: f64) -> f64 {
    let t98702 = -0.46336805555555555556e-3_f64 * t94626 * t98322 - 0.24872916666666666666e-2_f64 * t98673 + 0.88437037037037037034e-2_f64 * t98676 + 0.33163888888888888888e-2_f64 * t98680 + 0.73697530864197530862e-3_f64 * t98684 - 0.27802083333333333334e-2_f64 * t27459 * t28375 - 0.27802083333333333334e-2_f64 * t7908 * t28372 * t98205 * t1380 - 0.13901041666666666667e-2_f64 * t7908 * t28372 * t28373 * t4007 - 0.12356481481481481481e-2_f64 * t28392 * t27455 - 0.37069444444444444444e-2_f64 * t28480 * t7916 - 0.18534722222222222222e-2_f64 * t8151 * t27403;
    t98702
}
