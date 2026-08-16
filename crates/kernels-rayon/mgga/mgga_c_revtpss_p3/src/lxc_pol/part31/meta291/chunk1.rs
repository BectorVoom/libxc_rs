//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1278/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1278(t3943: f64, t794: f64, t1412: f64, t159: f64, t216: f64, t1408: f64, t2482: f64, t596: f64, t3981: f64, t212: f64, t225: f64, t816: f64) -> (f64, f64, f64, f64, f64) {
    let t9744 = t794 * t3943;
    let t9747 = t159 * t1412;
    let t9748 = t216 * t9747;
    let t9765 = t2482 * t1408 * t596;
    let t9766 = t9765 * t3981;
    let t9775 = t816 * t596 * t212 * t225;
    (t9744, t9748, t9765, t9766, t9775)
}
