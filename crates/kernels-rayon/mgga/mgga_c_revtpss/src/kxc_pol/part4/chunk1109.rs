//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1109/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1109(t9597: f64, t123: f64, t1856: f64, t2630: f64, t1857: f64, t3860: f64, t3863: f64, t13581: f64, t189: f64, t512: f64, t1907: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13664 = 12.0_f64 * t9597;
    let t13665 = t1856 * t123;
    let t13666 = t13665 * t2630;
    let t13667 = 0.10843581300301739842e-1_f64 * t13666;
    let t13668 = t3860 * t1857;
    let t13669 = 12.0_f64 * t13668;
    let t13670 = t3863 * t1857;
    let t13671 = 32.0_f64 * t13670;
    let t13672 = t13581 * t189;
    let t13673 = t512 * t13672;
    let t13674 = t1907 * t9593;
    (t13664, t13667, t13669, t13671, t13673, t13674)
}
