//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 241/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk241(t1088: f64, t1090: f64, t123: f64, t1087: f64, t423: f64, t419: f64, t409: f64, t410: f64, t1086: f64, t407: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1091 = t1088 * t1090;
    let t1092 = t123 * t1091;
    let t1094 = -t1087 + 0.17808333333333333333e-1_f64 * t1092;
    let t1096 = 0.621814e-1_f64 * t1094 * t423;
    let t1097 = t419 * t419;
    let t1098 = 1.0_f64 / t1097;
    let t1099 = t409 * t1098;
    let t1100 = 1.0_f64 / t410;
    let t1102 = -t1086 / 3.0_f64 + t1092 / 3.0_f64;
    let t1103 = t1100 * t1102;
    let t1105 = 0.29896666666666666667e0_f64 * t1086;
    let t1107 = f64::sqrt(t407);
    (t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100, t1102, t1103, t1105, t1107)
}
