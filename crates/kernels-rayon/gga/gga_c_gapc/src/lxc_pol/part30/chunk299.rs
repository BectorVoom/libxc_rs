//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 299/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk299(t132: f64, t425: f64, t391: f64, t88: f64, t69: f64, t62: f64, t402: f64, t106: f64, t19: f64, t65: f64, t20: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1134 = t132 * t425;
    let t1141 = t88 * t391;
    let t1144 = t69 * t69;
    let t1145 = 1.0_f64 / t1144;
    let t1146 = t62 * t1145;
    let t1147 = t402 * t402;
    let t1150 = 1.0_f64 / t106;
    let t1152 = t1150 * t65 * t19;
    let t1153 = t20 * t5;
    (t1134, t1141, t1146, t1147, t1150, t1152, t1153)
}
