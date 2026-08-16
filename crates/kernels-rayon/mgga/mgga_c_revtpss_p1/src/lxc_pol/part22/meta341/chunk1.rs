//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1813/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1813(t11132: f64, t2912: f64, t698: f64, t240: f64, t624: f64, t281: f64, t283: f64, t2909: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11304 = 28.0_f64 / 27.0_f64 * t11132;
    let t11326 = t698 * t2912;
    let t11334 = 0.93011851851851851854e0_f64 * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = 0.36514074074074074075e0_f64 * t11337;
    let t11339 = t698 * t2909;
    let t11341 = t240 * t3252;
    (t11304, t11326, t11334, t11335, t11337, t11338, t11339, t11341)
}
