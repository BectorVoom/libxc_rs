//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2976/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2976(t13792: f64, t48863: f64, t49137: f64, t13920: f64, t2661: f64, t3992: f64, t543: f64, t550: f64, t1398: f64, t5658: f64, t10073: f64, t14124: f64) -> (f64, f64, f64, f64) {
    let t49139 = t49137 * t48863 * t13792;
    let t49144 = t2661 * t3992 * t550 * t13920 * t543;
    let t49146 = t5658 * t1398;
    let t49167 = t10073 * t14124;
    (t49139, t49144, t49146, t49167)
}
