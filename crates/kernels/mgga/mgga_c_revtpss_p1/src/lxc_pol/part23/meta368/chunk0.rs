//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1690/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1690<F: Float>(t15670: F, t366: F, t3106: F, t4817: F, t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F) -> (F, F, F, F, F, F) {
    let t15671 = t15670 * t366;
    let t15675 = F::cast_from(0.10162730220579493208e-2_f64) * t3106 * t4817;
    let t15682 = t11710 * t4787;
    let t15684 = F::cast_from(0.19055119163586549765e-3_f64) * t3091 * t15682;
    let t15687 = t4890 * t245;
    let t15688 = t3088 * t15687;
    (t15671, t15675, t15682, t15684, t15687, t15688)
}
