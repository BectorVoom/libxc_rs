//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1021/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1021<F: Float>(t2990: F, t3: F, t10215: F, t10219: F, t10224: F, t2987: F, t2989: F, t2991: F, t8140: F, t8142: F, t8148: F, t8158: F, t8160: F, t8176: F, t8178: F, t8185: F, t8193: F, t8196: F, t8198: F) -> (F, F) {
    let t10231 = t2990 * t3;
    let t10238 = -t8158 - t2987 * t2989 * t10215 / 48.0 - t2987 * t10219 * t2991 / 24.0 + t8160 * t2989 * t10224 / 16.0 - 7.0 / 144.0 * t8140 * t8142 * t10224 - t2987 * t8148 * t10231 / 12.0 - t8176 - t8178 + t8185 / 48.0 - t8193 - t8196 / 16.0 + t8198 / 48.0;
    (t10231, t10238)
}
