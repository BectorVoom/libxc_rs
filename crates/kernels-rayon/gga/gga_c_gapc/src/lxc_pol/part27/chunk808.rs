//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 808/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk808(t7375: f64, t9399: f64, t2660: f64, t8911: f64, t129: f64, t8061: f64, t1078: f64, t8992: f64, t933: f64, t2600: f64, t8769: f64, t2629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9400 = t9399 * t7375;
    let t9402 = t2660 * t8911;
    let t9403 = t9402 * t7375;
    let t9405 = t8061 * t129;
    let t9406 = t9405 * t1078;
    let t9408 = t933 * t8992;
    let t9409 = t9408 * t2600;
    let t9411 = t933 * t8769;
    let t9412 = t9411 * t2629;
    (t9400, t9403, t9406, t9408, t9409, t9412)
}
