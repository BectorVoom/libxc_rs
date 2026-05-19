//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1267/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1267<F: Float>(t1380: F, t27403: F, t27455: F, t27459: F, t28372: F, t28373: F, t28375: F, t28392: F, t28480: F, t4007: F, t7908: F, t7916: F, t8151: F, t94626: F, t98205: F, t98322: F, t98673: F, t98676: F, t98680: F, t98684: F) -> F {
    let t98702 = -F::cast_from(0.46336805555555555556e-3_f64) * t94626 * t98322 - F::cast_from(0.24872916666666666666e-2_f64) * t98673 + F::cast_from(0.88437037037037037034e-2_f64) * t98676 + F::cast_from(0.33163888888888888888e-2_f64) * t98680 + F::cast_from(0.73697530864197530862e-3_f64) * t98684 - F::cast_from(0.27802083333333333334e-2_f64) * t27459 * t28375 - F::cast_from(0.27802083333333333334e-2_f64) * t7908 * t28372 * t98205 * t1380 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t28372 * t28373 * t4007 - F::cast_from(0.12356481481481481481e-2_f64) * t28392 * t27455 - F::cast_from(0.37069444444444444444e-2_f64) * t28480 * t7916 - F::cast_from(0.18534722222222222222e-2_f64) * t8151 * t27403;
    t98702
}
