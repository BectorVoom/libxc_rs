//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 929/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk929(t2085: f64, t4210: f64, t13299: f64, t31115: f64, t1988: f64, t7681: f64, t1095: f64, t1980: f64, t30058: f64, t3116: f64, t7310: f64, t7389: f64) -> (f64, f64, f64, f64, f64) {
    let t31116 = t2085 * t4210;
    let t31118 = t31115 * t13299 * t31116;
    let t31120 = t1988 * t7681;
    let t31124 = t1980 * t30058 * t1095 * t3116;
    let t31126 = t7310 * t7389;
    (t31116, t31118, t31120, t31124, t31126)
}
