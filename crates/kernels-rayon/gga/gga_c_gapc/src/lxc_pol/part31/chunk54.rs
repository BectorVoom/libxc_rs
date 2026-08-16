//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 54/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk54(t128: f64, t137: f64, t122: f64, t124: f64, t135: f64) -> (f64, f64, f64, f64, f64) {
    let t138 = t128 * t137;
    let t139 = 1.0_f64 / t122;
    let t140 = t139 * t124;
    let t141 = t138 * t140;
    let t144 = 30.0_f64 + 0.72806316506996704929e-2_f64 * t135 * t141;
    (t138, t139, t140, t141, t144)
}
