//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1284/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1284(t216: f64, t9747: f64, t3989: f64, t4014: f64, t221: f64, t3889: f64, t3979: f64, t3978: f64, t1408: f64, t2482: f64, t596: f64, t3981: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9748 = t216 * t9747;
    let t9753 = t3989 * t4014;
    let t9761 = t3979 * t221 * t3889;
    let t9762 = t3978 * t9761;
    let t9765 = t2482 * t1408 * t596;
    let t9766 = t9765 * t3981;
    (t9748, t9753, t9761, t9762, t9765, t9766)
}
