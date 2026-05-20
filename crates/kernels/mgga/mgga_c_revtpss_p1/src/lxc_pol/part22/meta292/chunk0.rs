//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1709/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1709<F: Float>(t3981: F, t9765: F, t3923: F, t550: F, t543: F, t3992: F, t2661: F, t212: F, t225: F, t596: F, t816: F) -> (F, F, F, F, F, F) {
    let t9766 = t9765 * t3981;
    let t9768 = t550 * t3923;
    let t9769 = t9768 * t543;
    let t9770 = t3992 * t9769;
    let t9771 = t2661 * t9770;
    let t9775 = t816 * t596 * t212 * t225;
    (t9766, t9768, t9769, t9770, t9771, t9775)
}
