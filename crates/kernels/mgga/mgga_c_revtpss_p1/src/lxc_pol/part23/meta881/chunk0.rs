//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2790/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2790<F: Float>(t2782: F, t4086: F, t543: F, t74982: F, t10073: F, t22373: F, t10069: F, t22369: F, t14216: F, t14239: F, t14220: F, t48007: F) -> (F, F, F, F, F) {
    let t74985 = t2782 * t4086 * t74982 * t543;
    let t74990 = t10073 * t22373;
    let t74999 = t10069 * t22369;
    let t75003 = t14239 * t14216;
    let t75005 = t48007 * t14220;
    (t74985, t74990, t74999, t75003, t75005)
}
