//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1013/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1013<F: Float>(t3943: F, t794: F, t1412: F, t159: F, t216: F, t1408: F, t2482: F, t596: F, t3981: F, t212: F, t225: F, t816: F) -> (F, F, F, F, F) {
    let t9744 = t794 * t3943;
    let t9747 = t159 * t1412;
    let t9748 = t216 * t9747;
    let t9765 = t2482 * t1408 * t596;
    let t9766 = t9765 * t3981;
    let t9775 = t816 * t596 * t212 * t225;
    (t9744, t9748, t9765, t9766, t9775)
}
