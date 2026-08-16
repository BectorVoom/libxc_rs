//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1388/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1388<F: Float>(t10293: F, t240: F, t243: F, t813: F, t816: F, t10675: F, t2689: F, t10777: F, t10779: F, t2706: F, t837: F, t798: F, t9726: F) -> (F, F, F, F, F) {
    let t40846 = t10293 * t240;
    let t40850 = F::cast_from(0.12516778469694349359e-1_f64) * t813 * t40846 * t243 * t816;
    let t40851 = t2689 * t10675;
    let t40855 = t10777 * t10779 * t2706 * t837;
    let t40861 = t9726 * t798;
    (t40846, t40850, t40851, t40855, t40861)
}
