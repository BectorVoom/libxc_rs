//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1999/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1999(t1455: f64, t7337: f64, t2045: f64, t4153: f64, t10301: f64, t607: f64, t1927: f64, t2248: f64, t1926: f64, t25163: f64, t6973: f64, t644: f64, t6977: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92559 = t1455 * t7337;
    let t92563 = t4153 * t2045;
    let t92565 = t10301 * t607;
    let t92569 = t1927 * t2248;
    let t92570 = t1926 * t92569;
    let t92573 = t6973 * t25163;
    let t92576 = t6977 * t644;
    (t92559, t92563, t92565, t92570, t92573, t92576)
}
