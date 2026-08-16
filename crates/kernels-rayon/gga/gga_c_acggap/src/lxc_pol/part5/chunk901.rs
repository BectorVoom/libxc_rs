//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 901/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk901(t111: f64, t13483: f64, t150: f64, t864: f64, t174: f64, t383: f64, t386: f64, t387: f64, t980: f64, t996: f64) -> (f64, f64, f64, f64, f64) {
    let t13484 = t111 * t13483;
    let t13485 = t13484 * t150;
    let t13487 = t864 * t864;
    let t13492 = 0.51448821741683684368e-2_f64 * t13485 * t383 * t386 * t387 * t174 * t13487;
    let t13502 = t980 * t996;
    (t13484, t13485, t13487, t13492, t13502)
}
