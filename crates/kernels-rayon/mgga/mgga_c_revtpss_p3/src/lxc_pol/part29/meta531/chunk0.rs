//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1860/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1860(t2247: f64, t2251: f64, t68: f64, t26205: f64, t6963: f64, t45972: f64, t7342: f64, t10309: f64, t26178: f64, t25159: f64, t2047: f64, t92569: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95310 = t2247 * t2251 * t68;
    let t95314 = t6963 * t26205;
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95320 = t95319 * t25159;
    let t95340 = t2047 * t92569;
    (t95310, t95314, t95316, t95319, t95320, t95340)
}
