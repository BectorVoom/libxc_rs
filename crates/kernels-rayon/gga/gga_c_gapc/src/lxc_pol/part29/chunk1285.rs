//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1285/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1285(t10346: f64, t11210: f64, t16677: f64, t19: f64, t6939: f64, t11626: f64, t3234: f64, t6179: f64, t11625: f64, t11669: f64, t2440: f64, t3728: f64, t7029: f64) -> (f64, f64, f64, f64) {
    let t35820 = t10346 * t6939 * t19 * t11210 * t16677;
    let t35823 = t3234 * t6179 * t11626;
    let t35826 = t11625 * t11669 * t2440;
    let t35829 = t11625 * t3728 * t7029;
    (t35820, t35823, t35826, t35829)
}
