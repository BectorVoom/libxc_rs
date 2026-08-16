//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 960/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk960(t3109: f64, t9129: f64, t1179: f64, t8508: f64, t8946: f64, t466: f64, t8529: f64, t10: f64, t1135: f64) -> (f64, f64, f64, f64, f64) {
    let t9176 = t9129 * t3109;
    let t9179 = t1179 * t8508;
    let t9181 = t1179 * t8946;
    let t9188 = 0.22391424203717421017e-2_f64 * t466 * t8529;
    let t9189 = t10 * t1135;
    (t9176, t9179, t9181, t9188, t9189)
}
