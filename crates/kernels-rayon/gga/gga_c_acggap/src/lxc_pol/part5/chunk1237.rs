//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1237/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1237(t6106: f64, t997: f64, t1886: f64, t3670: f64, t5681: f64, t1008: f64, t3372: f64, t5727: f64, t1165: f64, t12801: f64, t16559: f64, t5852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22621 = t997 * t6106;
    let t22623 = t3670 * t1886;
    let t22625 = t997 * t5681;
    let t22627 = t1008 * t5681;
    let t22633 = t3372 * t5727;
    let t22642 = t12801 * t1165 * t5852 * t16559;
    (t22621, t22623, t22625, t22627, t22633, t22642)
}
