//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1097/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1097(t40538: f64, t953: f64, t41756: f64, t11327: f64, t123: f64, t4961: f64, t864: f64, t2672: f64, t4937: f64, t7274: f64, t930: f64, t25412: f64, t2812: f64, t5025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42152 = t953 * t40538;
    let t42157 = t953 * t41756;
    let t42177 = t11327 * t123;
    let t42181 = t864 * t4961;
    let t42182 = t42181 * t2672;
    let t42382 = t930 * t7274 * t4937;
    let t42427 = t2812 * t25412 * t5025;
    (t42152, t42157, t42177, t42182, t42382, t42427)
}
