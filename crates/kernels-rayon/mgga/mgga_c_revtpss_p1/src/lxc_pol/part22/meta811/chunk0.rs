//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2914/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2914(t39515: f64, t4083: f64, t10043: f64, t9303: f64, t10139: f64, t281: f64, t4056: f64, t543: f64, t68: f64, t14192: f64, t555: f64, t10115: f64, t1441: f64) -> (f64, f64, f64, f64, f64) {
    let t47351 = 0.11564373972601816912e-1_f64 * t39515 * t4083;
    let t47352 = t9303 * t10043;
    let t47364 = t10139 * t281 * t68 * t4056 * t543;
    let t47371 = t14192 * t555;
    let t47381 = t10115 * t1441;
    (t47351, t47352, t47364, t47371, t47381)
}
