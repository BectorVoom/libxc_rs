//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1137/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1137(t1744: f64, t8709: f64, t15516: f64, t2660: f64, t11597: f64, t9574: f64, t9578: f64, t11917: f64, t3363: f64, t9846: f64, t11902: f64, t15938: f64) -> (f64, f64, f64, f64, f64) {
    let t34090 = t1744 * t8709;
    let t34092 = t2660 * t34090 * t15516;
    let t34095 = t9574 * t11597 * t9578;
    let t34098 = t3363 * t11917 * t9846;
    let t34100 = t11902 * t15938;
    (t34090, t34092, t34095, t34098, t34100)
}
