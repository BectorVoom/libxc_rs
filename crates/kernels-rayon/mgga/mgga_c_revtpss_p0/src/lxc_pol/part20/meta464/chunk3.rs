//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1767/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1767(t10043: f64, t9303: f64, t10014: f64, t10019: f64, t268: f64, t4101: f64, t543: f64, t675: f64, t9890: f64, t10139: f64, t281: f64, t4056: f64, t68: f64) -> (f64, f64, f64, f64) {
    let t47352 = t9303 * t10043;
    let t47354 = t10014 * t10019;
    let t47359 = t4101 * t268 * t675 * t9890 * t543;
    let t47364 = t10139 * t281 * t68 * t4056 * t543;
    (t47352, t47354, t47359, t47364)
}
