//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 814/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk814(t216: f64, t9747: f64, t1408: f64, t2482: f64, t596: f64, t212: f64, t225: f64, t816: f64, t2681: f64, t820: f64, t124: f64, t2237: f64, t800: f64) -> (f64, f64, f64, f64, f64) {
    let t9748 = t216 * t9747;
    let t9765 = t2482 * t1408 * t596;
    let t9775 = t816 * t596 * t212 * t225;
    let t9779 = t820 * t1408 * t2681;
    let t9784 = t800 * t124 * t2237 * t212;
    (t9748, t9765, t9775, t9779, t9784)
}
