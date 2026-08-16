//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1993/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1993(t1882: f64, t9994: f64, t13872: f64, t221: f64, t4056: f64, t13867: f64, t13824: f64, t1398: f64, t5658: f64, t48073: f64, t543: f64, t3923: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48105 = t1882 * t9994;
    let t48141 = t221 * t13872;
    let t48475 = t1882 * t4056;
    let t48525 = t221 * t13867;
    let t48662 = t221 * t13824;
    let t49146 = t5658 * t1398;
    let t49306 = t48073 * t543;
    let t49376 = t49146 * t543;
    let t49380 = t48475 * t543;
    let t49393 = t48105 * t3923;
    (t48141, t48525, t48662, t49306, t49376, t49380, t49393)
}
