//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 309/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk309<F: Float>(t7: F, t132: F, t1046: F, t492: F, t224: F, t544: F, t339: F, t674: F, t259: F, zeta_threshold: F) -> (F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t1048 = 4.0 * t1046 * t492;
    let t1051 = piecewise3(t8, 0.0, 4.0 / 3.0 * t224 * t544);
    let t1054 = piecewise3(t133, 0.0, 4.0 / 3.0 * t339 * t674);
    let t1056 = (t1051 + t1054) * t259;
    (t1048, t1056)
}
