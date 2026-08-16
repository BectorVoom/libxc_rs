//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 704/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk704(t2661: f64, t7371: f64, t2263: f64, t864: f64, t2548: f64, t7298: f64, t312: f64, t9: f64, t116: f64, t7328: f64, t286: f64, t2666: f64, t311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7386 = t2661 * t7371;
    let t7397 = t864 * t2263;
    let t7405 = t2548 * t7298;
    let t7433 = t9 * t312;
    let t7445 = t116 * t7328;
    let t7447 = 5.0_f64 / 1296.0_f64 * t286 * t7445;
    let t7448 = t2666 * t311;
    (t7386, t7397, t7405, t7433, t7447, t7448)
}
