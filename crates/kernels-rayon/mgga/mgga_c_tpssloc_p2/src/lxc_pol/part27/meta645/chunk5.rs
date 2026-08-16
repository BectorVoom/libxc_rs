//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2212/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2212(t1933: f64, t23479: f64, t88365: f64, t23562: f64, t25637: f64, t984: f64, t1014: f64, t82654: f64, t1022: f64, t14037: f64, t1611: f64, t23419: f64, t23556: f64, t25655: f64, t25661: f64, t363: f64, t378: f64, t6747: f64, t6800: f64, t7583: f64, t82971: f64, t82996: f64, t83085: f64, t88400: f64, t88407: f64, t88415: f64, t88422: f64, t88425: f64) -> f64 {
    let t88428 = 0.20186378047070195428e-3_f64 * t1933 * t88365 * t23479;
    let t88430 = t23562 * t25637 * t984;
    let t88431 = t82654 * t1014;
    let t88437 = -0.32298204875312312684e-2_f64 * t88400 * t25655 + 0.16149102437656156342e-2_f64 * t88400 * t25661 - 0.20186378047070195428e-3_f64 * t88407 * t6747 - 0.20186378047070195428e-3_f64 * t82971 + 19.0_f64 / 864.0_f64 * t1611 * t23556 * t378 - t88415 - 0.10093189023535097714e-3_f64 * t83085 * t7583 + 0.10093189023535097714e-3_f64 * t82996 + 5.0_f64 / 6912.0_f64 * t23419 * t14037 - t88422 - t88425 - t88428 - 0.20186378047070195428e-3_f64 * t88430 * t88431 * t363 * t1022 * t6800;
    t88437
}
