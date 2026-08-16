//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1822/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1822(t49146: f64, t543: f64, t48475: f64, t3923: f64, t48105: f64, t14304: f64, t4147: f64, t1868: f64, t4135: f64, t116: f64, t13424: f64, t10871: f64, t1558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49376 = t49146 * t543;
    let t49380 = t48475 * t543;
    let t49393 = t48105 * t3923;
    let t49564 = t14304 * t4147;
    let t49582 = t1868 * t4135;
    let t49686 = t13424 * t116;
    let t50474 = t1558 * t10871;
    (t49376, t49380, t49393, t49564, t49582, t49686, t50474)
}
