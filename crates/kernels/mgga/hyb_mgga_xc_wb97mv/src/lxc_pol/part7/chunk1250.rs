//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1250/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1250<F: Float>(t143: F, t10762: F, t10839: F, t1264: F, t1279: F, t172: F, t187: F, t2115: F, t2158: F, t30242: F, t30279: F, t30459: F, t30497: F, t30500: F, t30627: F, t30664: F, t3244: F, t3284: F, t4026: F, t4062: F, t739: F, t758: F, t8761: F, t8841: F) -> (F,) {
    let t144 = 0.135e1 <= t143;
    let t30669 = piecewise3(t144, t30242 + t30279 + t30459 + t30497, -8.0 / 3.0 * t30500 * t187 - 16.0 / 3.0 * t10762 * t758 - 8.0 / 3.0 * t4026 * t2158 - 16.0 / 3.0 * t8761 * t1279 - 32.0 / 3.0 * t3244 * t3284 - 16.0 / 3.0 * t1264 * t8841 - 8.0 / 3.0 * t2115 * t4062 - 16.0 / 3.0 * t739 * t10839 - 8.0 / 3.0 * t172 * (t30627 + t30664));
    (t30669,)
}
