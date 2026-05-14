//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1061/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1061<F: Float>(t9597: F, t123: F, t1856: F, t2630: F, t1857: F, t3860: F, t3863: F, t13581: F, t189: F, t512: F, t1907: F, t9593: F, t5566: F, t749: F, t9856: F, t1468: F, t9605: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13664 = 12.0 * t9597;
    let t13665 = t1856 * t123;
    let t13666 = t13665 * t2630;
    let t13667 = 0.10843581300301739842e-1 * t13666;
    let t13668 = t3860 * t1857;
    let t13669 = 12.0 * t13668;
    let t13670 = t3863 * t1857;
    let t13671 = 32.0 * t13670;
    let t13672 = t13581 * t189;
    let t13673 = t512 * t13672;
    let t13674 = t1907 * t9593;
    let t13680 = t5566 * t749;
    let t13682 = 2.0 * t512 * t13680;
    let t13683 = 48.0 * t9856;
    let t13687 = t9605 * t1468;
    (t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13687)
}
