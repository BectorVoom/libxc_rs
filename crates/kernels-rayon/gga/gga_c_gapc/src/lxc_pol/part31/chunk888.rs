//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 888/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk888(t10058: f64, t3325: f64, t134: f64, t2404: f64, t3412: f64, t3405: f64, t3411: f64, t2315: f64, t2801: f64, t6: f64, t3414: f64, t9722: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10059 = t3325 * t10058;
    let t10061 = t134 * t2404;
    let t10062 = t3412 * t10061;
    let t10063 = t3405 * t10062;
    let t10064 = t3411 * t10063;
    let t10067 = t134 * t2315;
    let t10068 = t2801 * t6 * t10067;
    let t10069 = t3405 * t10068;
    let t10070 = t3411 * t10069;
    let t10072 = t9722 * t3414;
    (t10059, t10063, t10064, t10069, t10070, t10072)
}
