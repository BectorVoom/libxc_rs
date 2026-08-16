//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1013/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1013(t11273: f64, t3160: f64, t2923: f64, t910: f64, t287: f64, t2922: f64, t275: f64, t11132: f64, t240: f64, t624: f64, t281: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11277 = t11273 * t3160;
    let t11294 = t910 * t2923;
    let t11298 = 1.0_f64 / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11304 = 28.0_f64 / 27.0_f64 * t11132;
    let t11334 = 0.93011851851851851854e0_f64 * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    (t11277, t11294, t11299, t11304, t11334, t11335, t11337)
}
