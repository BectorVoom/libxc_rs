//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 946/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk946<F: Float>(t10899: F, t216: F, t2729: F, t794: F, t2732: F, t136: F, t860: F, t2457: F, t2710: F, t10652: F, t231: F, t2783: F, t2782: F, t10069: F, t2786: F, t10073: F) -> (F, F, F, F, F, F, F) {
    let t10900 = t216 * t10899;
    let t10905 = t794 * t2729;
    let t10906 = t10905 * t2732;
    let t10914 = t860 * t136;
    let t10916 = t2710 * t10914 * t2457;
    let t10920 = t2783 * t10652 * t231;
    let t10921 = t2782 * t10920;
    let t10923 = t10069 * t2786;
    let t10925 = t10073 * t2786;
    (t10900, t10905, t10906, t10916, t10921, t10923, t10925)
}
