//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 911/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk911<F: Float>(t3989: F, t4014: F, t221: F, t3889: F, t3979: F, t3978: F, t1408: F, t2482: F, t596: F, t3981: F, t3923: F, t550: F, t543: F, t3992: F, t2661: F, t212: F, t225: F, t816: F) -> (F, F, F, F, F, F, F) {
    let t9753 = t3989 * t4014;
    let t9761 = t3979 * t221 * t3889;
    let t9762 = t3978 * t9761;
    let t9765 = t2482 * t1408 * t596;
    let t9766 = t9765 * t3981;
    let t9768 = t550 * t3923;
    let t9769 = t9768 * t543;
    let t9770 = t3992 * t9769;
    let t9771 = t2661 * t9770;
    let t9775 = t816 * t596 * t212 * t225;
    (t9753, t9762, t9765, t9766, t9768, t9771, t9775)
}
