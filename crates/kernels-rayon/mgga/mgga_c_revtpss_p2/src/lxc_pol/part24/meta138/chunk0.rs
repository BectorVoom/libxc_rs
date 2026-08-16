//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 720/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk720(t5618: f64, t807: f64, t1868: f64, t221: f64, t3979: f64, t3978: f64, t1885: f64, t3930: f64, t1856: f64, t72: f64, t757: f64, t539: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5619 = t807 * t5618;
    let t5622 = t3979 * t221 * t1868;
    let t5623 = t3978 * t5622;
    let t5625 = t3930 * t1885;
    let t5635 = t1856 * t72;
    let t5636 = t5635 * t757;
    let t5650 = t539 * t73;
    (t5619, t5622, t5623, t5625, t5635, t5636, t5650)
}
