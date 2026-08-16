//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2727/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2727<F: Float>(t17708: F, t59948: F, t17394: F, t370: F, t17727: F, t12916: F, t21258: F, t3718: F, t17753: F, t21045: F, t12866: F, t5401: F, t58895: F) -> (F, F, F, F, F, F) {
    let t70639 = t59948 * t17708;
    let t70646 = t17394 * t370;
    let t70647 = t17727 * t70646;
    let t70664 = t3718 * t12916 * t21258;
    let t70667 = t17753 * t12916 * t21045;
    let t70672 = t12866 * t58895 * t5401;
    (t70639, t70646, t70647, t70664, t70667, t70672)
}
