//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 516/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk516(t3101: f64, t452: f64, t381: f64, t136: f64, t2015: f64, t357: f64, t1074: f64, t2035: f64, t1059: f64, t576: f64, t1062: f64, t134: f64, t154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3102 = t452 * t3101;
    let t3104 = 0.65854491829355115987e0_f64 * t381 * t3102;
    let t3106 = t2015 * t136 * t357;
    let t3107 = 20.0_f64 / 27.0_f64 * t3106;
    let t3109 = t2035 * t136 * t1074;
    let t3110 = 2.0_f64 / 3.0_f64 * t3109;
    let t3111 = t576 * t1059;
    let t3112 = t3111 * t1062;
    let t3114 = t134 * t154;
    (t3104, t3106, t3107, t3109, t3110, t3111, t3112, t3114)
}
