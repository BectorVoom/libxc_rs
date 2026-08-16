//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 562/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk562(t441: f64, t851: f64, t323: f64, t1222: f64, t857: f64, t852: f64, t872: f64, t1221: f64, t322: f64, t1220: f64, t316: f64, t3101: f64, t317: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3901 = t851 * t441;
    let t3902 = t3901 * t323;
    let t3904 = t857 * t1222;
    let t3906 = t852 * t872;
    let t3908 = t322 * t1221;
    let t3909 = t1220 * t3908;
    let t3910 = t316 * t3909;
    let t3912 = t317 * t3101;
    (t3902, t3904, t3906, t3909, t3910, t3912)
}
