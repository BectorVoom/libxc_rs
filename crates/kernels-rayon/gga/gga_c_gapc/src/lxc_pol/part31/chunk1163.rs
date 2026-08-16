//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1163/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1163(t33374: f64, t7595: f64, t15553: f64, t15555: f64, t33287: f64, t33158: f64, t3402: f64, t3408: f64, t1084: f64, t11428: f64, t11927: f64, t1461: f64, t818: f64) -> (f64, f64, f64, f64) {
    let t33507 = t33374 * t7595;
    let t33510 = t15553 * t33287 * t15555;
    let t33513 = t3402 * t33158 * t3408;
    let t33518 = t1084 * t1461 * t11428 * t818 * t11927;
    (t33507, t33510, t33513, t33518)
}
