//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 989/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk989(t1028: f64, t1123: f64, t2269: f64, t297: f64, t2849: f64, t438: f64, t2855: f64, t1027: f64, t3107: f64, t2329: f64, t302: f64, t2434: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12743 = t1123 * t1028;
    let t14330 = t297 * t2269;
    let t15305 = t438 * t2849;
    let t15654 = t438 * t2855;
    let t17919 = t3107 * t1027;
    let t18485 = t2329 * t302;
    let t18634 = t2434 * t875;
    (t12743, t14330, t15305, t15654, t17919, t18485, t18634)
}
