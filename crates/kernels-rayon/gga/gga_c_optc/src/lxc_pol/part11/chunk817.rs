//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 817/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk817(t2849: f64, t438: f64, t1135: f64, t5328: f64, t19: f64, t2586: f64, t5301: f64, t1133: f64, t4369: f64, t4380: f64, t309: f64, t5279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15305 = t438 * t2849;
    let t15310 = t1135 * t5328;
    let t15311 = t15310 * t19;
    let t15321 = t2586 * t5301;
    let t15322 = t1133 * t15321;
    let t15324 = t4369 * t4380;
    let t15326 = t5279 * t309;
    (t15305, t15311, t15321, t15322, t15324, t15326)
}
