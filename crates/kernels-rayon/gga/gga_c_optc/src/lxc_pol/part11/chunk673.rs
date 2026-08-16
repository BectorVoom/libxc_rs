//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 673/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk673(t1810: f64, t571: f64, t1827: f64, t70: f64, t572: f64, t1824: f64, t67: f64, t62: f64, t1828: f64, t1863: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6389 = t1810 * t571;
    let t6391 = 1.0_f64 / t1827 / t70;
    let t6392 = t6389 * t6391;
    let t6395 = t6389 * t572;
    let t6399 = 1.0_f64 / t1824 / t67;
    let t6400 = t62 * t6399;
    let t6401 = t6389 * t1828;
    let t6405 = 1.0_f64 / t1863 / t80;
    (t6391, t6392, t6395, t6399, t6400, t6401, t6405)
}
