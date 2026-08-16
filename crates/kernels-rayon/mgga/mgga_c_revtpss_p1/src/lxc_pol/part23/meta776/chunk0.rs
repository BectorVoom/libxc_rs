//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2580/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2580(t3495: f64, t5155: f64, t3476: f64, t5117: f64, t3451: f64, t3383: f64, t5060: f64, t12247: f64, t1719: f64, t1756: f64, t3521: f64, t56228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t58307 = t5155 * t3495;
    let t58317 = t5117 * t3476;
    let t58336 = t5117 * t3451;
    let t58339 = t5060 * t3383;
    let t58342 = t1719 * t12247;
    let t58345 = t3521 * t1756;
    let t58404 = 0.40256666666666666668e0_f64 * t56228;
    (t58307, t58317, t58336, t58339, t58342, t58345, t58404)
}
