//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 989/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk989(t1398: f64, t215: f64, t268: f64, t543: f64, t4101: f64, t2453: f64, t4100: f64, t281: f64, t68: f64, t1357: f64, t4078: f64, t689: f64) -> (f64, f64, f64, f64) {
    let t10136 = t268 * t215 * t1398 * t543;
    let t10137 = t4101 * t10136;
    let t10139 = t2453 * t4100;
    let t10142 = t281 * t68 * t1398 * t543;
    let t10143 = t10139 * t10142;
    let t10150 = t1357 * t4078;
    let t10151 = t689 * t10150;
    (t10137, t10139, t10143, t10151)
}
