//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 965/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk965(t2132: f64, t322: f64, t7896: f64, t7979: f64, t2159: f64, t7924: f64, t310: f64, t7970: f64, t16548: f64, t7932: f64, t7942: f64, t2131: f64, t2147: f64, t463: f64) -> (f64, f64, f64, f64, f64) {
    let t31976 = t7896 * t2132 * t7979 * t322;
    let t31978 = t7924 * t2159;
    let t31984 = t310 * t7970;
    let t31991 = t7942 * t7932 * t16548;
    let t31999 = t2131 * t2147 * t7979 * t463;
    (t31976, t31978, t31984, t31991, t31999)
}
