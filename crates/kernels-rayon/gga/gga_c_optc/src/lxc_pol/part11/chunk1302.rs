//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1302/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1302(t265: f64, t57197: f64, t57209: f64, t241: f64, t1343: f64, t50721: f64, t14267: f64, t4856: f64, t10493: f64, t16674: f64, t1342: f64, t16858: f64, t2373: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57211 = (t57197 + t57209) * t265;
    let t57213 = 0.19751789702565206229e-1_f64 * t241 * t57211;
    let t57215 = 4.0_f64 * t50721 * t1343;
    let t57217 = 0.70178680769462448852e1_f64 * t14267 * t4856;
    let t57219 = 0.19298189186581325787e3_f64 * t10493 * t16674;
    let t57222 = 8.0_f64 * t2373 * t16858 * t1342;
    (t57211, t57213, t57215, t57217, t57219, t57222)
}
