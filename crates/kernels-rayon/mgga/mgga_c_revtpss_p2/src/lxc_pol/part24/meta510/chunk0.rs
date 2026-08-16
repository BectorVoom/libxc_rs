//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1525/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1525(t23754: f64, t2970: f64, t23694: f64, t3014: f64, t23546: f64, t2926: f64, t3011: f64, t24186: f64, t3336: f64, t11249: f64, t23640: f64, t15926: f64, t19976: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78165 = t23754 * t2970;
    let t78207 = t23694 * t3014;
    let t78329 = t23546 * t2926;
    let t78429 = t3011 * t23694;
    let t78478 = t24186 * t3336;
    let t78496 = t23640 * t11249;
    let t78512 = t15926 * t19976;
    (t78165, t78207, t78329, t78429, t78478, t78496, t78512)
}
