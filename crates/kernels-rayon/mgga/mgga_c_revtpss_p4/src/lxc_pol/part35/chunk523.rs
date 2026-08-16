//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 523/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk523(t3992: f64, t5609: f64, t2661: f64, t1413: f64, t1868: f64, t547: f64, t807: f64, t221: f64, t3979: f64, t3978: f64, t1885: f64, t3930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5610 = t3992 * t5609;
    let t5611 = t2661 * t5610;
    let t5617 = t1413 * t1868;
    let t5618 = t547 * t5617;
    let t5619 = t807 * t5618;
    let t5622 = t3979 * t221 * t1868;
    let t5623 = t3978 * t5622;
    let t5625 = t3930 * t1885;
    (t5610, t5611, t5617, t5618, t5619, t5622, t5623, t5625)
}
