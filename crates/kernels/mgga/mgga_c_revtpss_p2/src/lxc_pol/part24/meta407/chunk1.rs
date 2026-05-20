//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1348/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1348<F: Float>(t243: F, t40846: F, t813: F, t816: F, t798: F, t9726: F, t10899: F, t794: F, t159: F, t216: F, t2475: F, t251: F, t40321: F) -> (F, F, F, F, F) {
    let t40850 = F::cast_from(0.12516778469694349359e-1_f64) * t813 * t40846 * t243 * t816;
    let t40861 = t9726 * t798;
    let t40864 = t794 * t10899;
    let t40868 = t216 * t159 * t2475;
    let t40902 = t40321 * t251;
    (t40850, t40861, t40864, t40868, t40902)
}
