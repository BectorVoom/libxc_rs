//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1057/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1057<F: Float>(t7: F, t3864: F, t6768: F, t2181: F, t3854: F, t10273: F, t1875: F, t3319: F, t544: F, t775: F, t222: F, t37: F, zeta_threshold: F) -> (F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t10965 = t6768 * t3864;
    let t10970 = t2181 * t3854;
    let t10976 = piecewise3(t8, 0.0, -28.0 / 27.0 * t10965 * t544 + 16.0 / 9.0 * t3319 * t1875 + 4.0 / 9.0 * t10970 * t544 - t775 * t10273 / 3.0);
    let t10978 = t222 * t37 * t10976;
    (t10965, t10970, t10976, t10978)
}
