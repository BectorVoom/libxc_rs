//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1019/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1019<F: Float>(t2496: F, t5571: F, t9597: F, t123: F, t1856: F, t2630: F, t1857: F, t3860: F, t5566: F, t749: F, t512: F, t9856: F, t1892: F, t785: F, t1358: F, t2439: F) -> (F, F, F, F, F, F, F) {
    let t13652 = t5571 * t2496;
    let t13664 = 12.0 * t9597;
    let t13665 = t1856 * t123;
    let t13666 = t13665 * t2630;
    let t13668 = t3860 * t1857;
    let t13680 = t5566 * t749;
    let t13682 = 2.0 * t512 * t13680;
    let t13683 = 48.0 * t9856;
    let t13725 = t785 * t1892;
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    (t13652, t13664, t13666, t13668, t13682, t13683, t13727)
}
