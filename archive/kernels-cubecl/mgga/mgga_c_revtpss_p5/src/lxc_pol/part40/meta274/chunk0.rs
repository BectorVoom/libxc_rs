//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1013/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1013<F: Float>(t3923: F, t550: F, t543: F, t3992: F, t2661: F, t212: F, t225: F, t596: F, t816: F, t3995: F, t1408: F, t2681: F, t820: F) -> (F, F, F, F, F) {
    let t9768 = t550 * t3923;
    let t9769 = t9768 * t543;
    let t9770 = t3992 * t9769;
    let t9771 = t2661 * t9770;
    let t9775 = t816 * t596 * t212 * t225;
    let t9776 = t9775 * t3995;
    let t9779 = t820 * t1408 * t2681;
    (t9768, t9771, t9775, t9776, t9779)
}
