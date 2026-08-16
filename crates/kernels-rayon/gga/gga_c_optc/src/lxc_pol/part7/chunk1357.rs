//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1357/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1357(t1: f64, t27010: f64, t15654: f64, t9044: f64, t123: f64, t17919: f64, t1900: f64, t15305: f64, t2860: f64, t4356: f64, t24502: f64, t3102: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27011 = t27010 * t1;
    let t27012 = t15654 * t9044;
    let t27017 = t17919 * t1900 * t123;
    let t27023 = t15305 * t9044;
    let t27027 = t4356 * t2860;
    let t27031 = t3102 * t24502;
    (t27011, t27012, t27017, t27023, t27027, t27031)
}
