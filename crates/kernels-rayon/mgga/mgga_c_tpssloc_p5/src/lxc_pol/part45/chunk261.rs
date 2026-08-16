//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 261/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk261(t1136: f64, t1137: f64, t1086: f64, t1092: f64, t449: f64, t445: f64, t440: f64, t1111: f64, t1103: f64, t1108: f64, t1115: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1138 = t1136 * t1137;
    let t1141 = 0.92708333333333333333e-2_f64 * t1086;
    let t1143 = -t1141 + 0.92708333333333333333e-2_f64 * t1092;
    let t1144 = t1143 * t449;
    let t1146 = t445 * t445;
    let t1147 = 1.0_f64 / t1146;
    let t1148 = t440 * t1147;
    let t1150 = 0.301925e0_f64 * t1086;
    let t1153 = 0.82785e-1_f64 * t1111;
    let t1155 = 0.258925e1_f64 * t1103 - t1150 + 0.301925e0_f64 * t1092 + 0.16504875e0_f64 * t1108 - t1153 + 0.82785e-1_f64 * t1115;
    let t1156 = 1.0_f64 / t448;
    (t1138, t1143, t1144, t1146, t1147, t1148, t1155, t1156)
}
