//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1115/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1115(t3146: f64, t4573: f64, t2667: f64, t3101: f64, t5255: f64, t5416: f64, t1111: f64, t5285: f64, t530: f64, t3151: f64, t26910: f64, t5328: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46469 = t3146 * t4573;
    let t46536 = t3101 * t5255 * t2667;
    let t46539 = t5416 * t2667;
    let t46590 = t1111 * t530 * t5285;
    let t46697 = t3151 * t4573;
    let t46715 = t26910 * t5328;
    (t46469, t46536, t46539, t46590, t46697, t46715)
}
