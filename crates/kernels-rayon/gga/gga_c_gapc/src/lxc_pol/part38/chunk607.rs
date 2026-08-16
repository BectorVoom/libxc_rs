//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 607/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk607(t1023: f64, t3670: f64, t128: f64, t1457: f64, t1033: f64, t169: f64, t3157: f64, t1044: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3671 = t3670 * t1023;
    let t3673 = t1457 * t128;
    let t3674 = t3673 * t1033;
    let t3676 = t169 * t3674 * t3157;
    let t3678 = t128 * t1044;
    let t3679 = t3678 * pi;
    (t3671, t3673, t3674, t3676, t3679)
}
