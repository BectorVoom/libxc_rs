//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 363/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk363(t432: f64, t427: f64, t1086: f64, t1111: f64, t1092: f64, t1103: f64, t1108: f64, t1115: f64) -> (f64, f64, f64, f64) {
    let t1127 = t432 * t432;
    let t1128 = 1.0_f64 / t1127;
    let t1129 = t427 * t1128;
    let t1131 = 0.516475e0_f64 * t1086;
    let t1134 = 0.104195e0_f64 * t1111;
    let t1136 = 0.3529725e1_f64 * t1103 - t1131 + 0.516475e0_f64 * t1092 + 0.6311625e0_f64 * t1108 - t1134 + 0.104195e0_f64 * t1115;
    (t1127, t1128, t1129, t1136)
}
