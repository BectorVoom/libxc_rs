//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1255/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1255(t1646: f64, t1651: f64, t29807: f64, t994: f64, t1647: f64, t7810: f64, t1078: f64, t1982: f64, t3140: f64, t6343: f64, t29894: f64, t3336: f64) -> (f64, f64, f64, f64, f64) {
    let t107532 = t1646 * t1651;
    let t107566 = t994 * t29807;
    let t107629 = t1647 * t7810;
    let t107636 = t1982 * t6343 * t3140 * t1078;
    let t107741 = t29894 * t3336;
    (t107532, t107566, t107629, t107636, t107741)
}
