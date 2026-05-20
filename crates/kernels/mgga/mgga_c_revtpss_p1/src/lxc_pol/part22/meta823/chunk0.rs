//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2939/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2939<F: Float>(t14220: F, t46495: F, t4086: F, t5710: F, t786: F, t4104: F, t1437: F, t2482: F, t5658: F, t2782: F, t48015: F, t543: F) -> (F, F, F, F, F) {
    let t48041 = t46495 * t14220;
    let t48048 = t786 * t4086 * t5710;
    let t48049 = t48048 * t4104;
    let t48058 = t2482 * t1437 * t5658 * t4104;
    let t48066 = t2782 * t4086 * t48015 * t543;
    (t48041, t48048, t48049, t48058, t48066)
}
