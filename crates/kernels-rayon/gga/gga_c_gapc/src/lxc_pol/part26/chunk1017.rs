//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1017/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1017(t144: f64, t3707: f64, t5972: f64, t647: f64, t137: f64, t5: f64, t4: f64, t5971: f64, t11589: f64, t102: f64, t198: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20461 = t3707 * t144;
    let t20487 = t647 * t5972;
    let t20499 = t5 * t137;
    let t20500 = t20499 * t4;
    let t20501 = t5971 * t20500;
    let t20563 = t11589 * t137;
    let t20569 = t102 * t198 * t674;
    (t20461, t20487, t20500, t20501, t20563, t20569)
}
