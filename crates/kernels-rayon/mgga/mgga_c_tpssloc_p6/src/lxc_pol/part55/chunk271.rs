//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 271/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk271(t1092: f64, t1122: f64, t432: f64, t427: f64, t1086: f64, t1111: f64, t1103: f64, t1108: f64, t1115: f64, t435: f64, t449: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1124 = -t1122 + 0.17123333333333333333e-1_f64 * t1092;
    let t1127 = t432 * t432;
    let t1128 = 1.0_f64 / t1127;
    let t1129 = t427 * t1128;
    let t1131 = 0.516475e0_f64 * t1086;
    let t1134 = 0.104195e0_f64 * t1111;
    let t1136 = 0.3529725e1_f64 * t1103 - t1131 + 0.516475e0_f64 * t1092 + 0.6311625e0_f64 * t1108 - t1134 + 0.104195e0_f64 * t1115;
    let t1137 = 1.0_f64 / t435;
    let t1138 = t1136 * t1137;
    let t1141 = 0.92708333333333333333e-2_f64 * t1086;
    let t1143 = -t1141 + 0.92708333333333333333e-2_f64 * t1092;
    let t1144 = t1143 * t449;
    (t1124, t1127, t1128, t1129, t1131, t1134, t1136, t1137, t1138, t1141, t1143, t1144)
}
