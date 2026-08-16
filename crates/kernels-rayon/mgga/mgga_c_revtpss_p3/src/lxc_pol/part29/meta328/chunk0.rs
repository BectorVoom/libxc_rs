//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1239/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1239(t2461: f64, t2471: f64, t788: f64, t9288: f64, t787: f64, t2453: f64, t861: f64, t2458: f64, t2761: f64, t786: f64, t789: f64, t212: f64, t2760: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11013 = t2461 * t2471;
    let t11015 = t788 * t9288;
    let t11017 = 0.30356481678079769392e-1_f64 * t787 * t11015;
    let t11018 = t2453 * t861;
    let t11019 = t11018 * t2458;
    let t11021 = t786 * t2761;
    let t11022 = t11021 * t789;
    let t11024 = t212 * t2760;
    (t11013, t11015, t11017, t11019, t11022, t11024)
}
