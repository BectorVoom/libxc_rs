//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2972/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2972<F: Float>(t13910: F, t808: F, t9736: F, t14026: F, t9744: F, t13821: F, t13999: F, t13716: F, t1413: F, t547: F, t807: F, t550: F, t9794: F) -> (F, F, F, F, F) {
    let t49056 = t9736 * t808 * t13910;
    let t49058 = t9744 * t14026;
    let t49062 = t13999 * t13821;
    let t49066 = t807 * t547 * t1413 * t13716;
    let t49068 = t9794 * t550;
    (t49056, t49058, t49062, t49066, t49068)
}
