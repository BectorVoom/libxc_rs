//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1026/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1026<F: Float>(t4004: F, t5673: F, t5674: F, t9840: F, t1868: F, t3829: F, t828: F, t9942: F, t5608: F, t5675: F, t9934: F, t2661: F, t3936: F, t5704: F, t3924: F, t2482: F, t4000: F, t814: F) -> (F, F, F, F, F, F, F) {
    let t13817 = t5673 * t5674 * t4004;
    let t13821 = t5673 * t5674 * t9840;
    let t13824 = t1868 * t3829;
    let t13826 = t9942 * t828 * t13824;
    let t13829 = t5608 * t5675;
    let t13830 = t9934 * t13829;
    let t13832 = 0.28582678745379824648e-4 * t2661 * t13830;
    let t13834 = t3936 * t5704 * t4004;
    let t13841 = t3936 * t5704 * t3924;
    let t13845 = t2482 * t4000 * t814;
    (t13817, t13821, t13826, t13832, t13834, t13841, t13845)
}
