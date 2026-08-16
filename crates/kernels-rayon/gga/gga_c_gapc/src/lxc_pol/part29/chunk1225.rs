//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1225/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1225(t11303: f64, t20200: f64, t27307: f64, t27309: f64, t33399: f64, t8362: f64, t19639: f64, t34317: f64, t1030: f64, t3008: f64, t33158: f64, t34447: f64, t3949: f64, t9203: f64) -> (f64, f64, f64, f64, f64) {
    let t35115 = t11303 * t20200;
    let t35119 = t27307 * t33399 * t8362 * t27309;
    let t35121 = t34317 * t19639;
    let t35124 = t1030 * t33158 * t3008;
    let t35127 = t9203 * t34447 * t3949;
    (t35115, t35119, t35121, t35124, t35127)
}
