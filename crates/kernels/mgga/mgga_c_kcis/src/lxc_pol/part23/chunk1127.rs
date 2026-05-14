//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1127/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1127<F: Float>(t1394: F, t27427: F, t28356: F, t1464: F, t2046: F, t28503: F, t4124: F, t4136: F, t51613: F, t7923: F, t1380: F, t27403: F, t27455: F, t27459: F, t28372: F, t28373: F, t28375: F, t28392: F, t28480: F, t4007: F, t7908: F, t7916: F, t8151: F, t94626: F, t98205: F, t98322: F, t98673: F) -> (F, F, F, F) {
    let t98676 = t1394 * t28356 * t27427;
    let t98680 = t1464 * t28503 * t2046 * t4124;
    let t98684 = t1464 * t7923 * t51613 * t4136;
    let t98702 = -0.46336805555555555556e-3 * t94626 * t98322 - 0.24872916666666666666e-2 * t98673 + 0.88437037037037037034e-2 * t98676 + 0.33163888888888888888e-2 * t98680 + 0.73697530864197530862e-3 * t98684 - 0.27802083333333333334e-2 * t27459 * t28375 - 0.27802083333333333334e-2 * t7908 * t28372 * t98205 * t1380 - 0.13901041666666666667e-2 * t7908 * t28372 * t28373 * t4007 - 0.12356481481481481481e-2 * t28392 * t27455 - 0.37069444444444444444e-2 * t28480 * t7916 - 0.18534722222222222222e-2 * t8151 * t27403;
    (t98676, t98680, t98684, t98702)
}
