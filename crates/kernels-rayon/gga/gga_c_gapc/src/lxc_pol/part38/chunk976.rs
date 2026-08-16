//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 976/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk976(t3750: f64, t871: f64, t3388: f64, t3392: f64, t3751: f64, t3769: f64, t949: f64, t1084: f64, t11430: f64, t10079: f64, t11597: f64, t3402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11862 = t871 * t3750;
    let t11863 = t11862 * t3388;
    let t11865 = t3751 * t3392;
    let t11867 = t3769 * t949;
    let t11869 = t1084 * t11430;
    let t11870 = t11869 * t10079;
    let t11872 = t3402 * t11597;
    (t11862, t11863, t11865, t11867, t11869, t11870, t11872)
}
