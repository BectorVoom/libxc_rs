//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 957/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk957(t3237: f64, t9142: f64, t3244: f64, t2367: f64, t3093: f64, t1162: f64, t8538: f64, t914: f64, t1179: f64, t8505: f64, t8521: f64, t3126: f64, t9073: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9143 = t9142 * t3237;
    let t9144 = t3244 * t9143;
    let t9148 = t2367 * t3093;
    let t9149 = t1162 * t9148;
    let t9151 = t914 * t8538;
    let t9156 = t1179 * t8505;
    let t9158 = t1179 * t8521;
    let t9160 = t9073 * t3126;
    (t9144, t9149, t9151, t9156, t9158, t9160)
}
