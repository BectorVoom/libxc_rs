//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 215/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk215(t1127: f64, t427: f64, t1086: f64, t1111: f64, t435: f64, t445: f64, t440: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1128 = 1.0_f64 / t1127;
    let t1129 = t427 * t1128;
    let t1131 = 0.516475e0_f64 * t1086;
    let t1134 = 0.104195e0_f64 * t1111;
    let t1137 = 1.0_f64 / t435;
    let t1141 = 0.92708333333333333333e-2_f64 * t1086;
    let t1146 = t445 * t445;
    let t1147 = 1.0_f64 / t1146;
    let t1148 = t440 * t1147;
    let t1150 = 0.301925e0_f64 * t1086;
    let t1153 = 0.82785e-1_f64 * t1111;
    let t1156 = 1.0_f64 / t448;
    (t1128, t1129, t1131, t1134, t1137, t1141, t1146, t1147, t1148, t1150, t1153, t1156)
}
