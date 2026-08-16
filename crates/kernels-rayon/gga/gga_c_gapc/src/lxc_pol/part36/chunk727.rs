//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 727/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk727(t1932: f64, t2972: f64, t2975: f64, t126: f64, t1762: f64, t611: f64, t2979: f64, t3085: f64, t1: f64, t5011: f64, t5541: f64, t102: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8662 = t1932 * t2972;
    let t8663 = t8662 * t2975;
    let t8666 = t126 * t1762;
    let t8667 = t611 * t8666;
    let t8668 = t8667 * t2975;
    let t8670 = t1932 * t2979;
    let t8671 = t8670 * t3085;
    let t8673 = t5011 * t1;
    let t8674 = t5541 * t8673;
    let t8675 = t102 * t6;
    (t8663, t8666, t8668, t8671, t8673, t8674, t8675)
}
