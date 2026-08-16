//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1253/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1253(t25411: f64, t93225: f64, t1959: f64, t41117: f64, t68: f64, t785: f64, t251: f64, t281: f64, t25410: f64, t7078: f64, t10910: f64, t1955: f64) -> (f64, f64, f64, f64, f64) {
    let t93228 = t25411 * t93225;
    let t93231 = 0.81814717454467823679e-4_f64 * t41117 * t1959;
    let t93238 = t68 * t785;
    let t93240 = t281 * t93238 * t251;
    let t93242 = t93240 * t25410 * t7078;
    let t93244 = t1955 * t10910;
    (t93228, t93231, t93238, t93242, t93244)
}
