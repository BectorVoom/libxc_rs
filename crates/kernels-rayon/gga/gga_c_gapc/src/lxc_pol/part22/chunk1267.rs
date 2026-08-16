//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1267/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1267(t19639: f64, t34317: f64, t1030: f64, t3008: f64, t33158: f64, t34447: f64, t3949: f64, t9203: f64, t128: f64, t3141: f64, t33655: f64, t5541: f64, t583: f64) -> (f64, f64, f64, f64) {
    let t35121 = t34317 * t19639;
    let t35124 = t1030 * t33158 * t3008;
    let t35127 = t9203 * t34447 * t3949;
    let t35132 = t5541 * t33655 * t3141 * t583 * t128;
    (t35121, t35124, t35127, t35132)
}
