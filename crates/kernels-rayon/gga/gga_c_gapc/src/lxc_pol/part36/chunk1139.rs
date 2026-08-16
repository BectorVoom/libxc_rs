//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1139/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1139(t10293: f64, t28192: f64, t33399: f64, t9894: f64, t1744: f64, t8709: f64, t15516: f64, t2660: f64, t11597: f64, t9574: f64, t9578: f64, t11917: f64, t3363: f64, t9846: f64) -> (f64, f64, f64, f64, f64) {
    let t34088 = t9894 * t33399 * t10293 * t28192;
    let t34090 = t1744 * t8709;
    let t34092 = t2660 * t34090 * t15516;
    let t34095 = t9574 * t11597 * t9578;
    let t34098 = t3363 * t11917 * t9846;
    (t34088, t34090, t34092, t34095, t34098)
}
