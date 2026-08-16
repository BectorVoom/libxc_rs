//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1243/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1243<F: Float>(t19456: F, t247: F, t3116: F, t3172: F, t6311: F, t3161: F, t1043: F, t6244: F, t1045: F, t3117: F, t1668: F, t4772: F) -> (F, F, F, F, F) {
    let t19819 = t247 * t3116 * t19456;
    let t19826 = t3172 * t6311;
    let t19827 = t3161 * t19826;
    let t19829 = t6244 * t1043;
    let t19830 = t19829 * t1045;
    let t19831 = t3117 * t19830;
    let t19836 = t4772 * t1668;
    (t19819, t19827, t19829, t19831, t19836)
}
