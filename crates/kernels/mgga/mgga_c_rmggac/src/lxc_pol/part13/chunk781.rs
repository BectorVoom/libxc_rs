//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 781/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk781<F: Float>(t7778: F, t866: F, t305: F, t2067: F, t25525: F, t2078: F, t3839: F, t262: F, t35917: F, t7785: F, t35844: F, t7788: F) -> (F, F, F, F, F, F, F, F) {
    let t36247 = t7778 * t866;
    let t36248 = t305 * t36247;
    let t36250 = t25525 * t2067;
    let t36254 = t3839 * t2078;
    let t36268 = t262 * t35917;
    let t36269 = t7785 * t36268;
    let t36271 = t262 * t35844;
    let t36272 = t7788 * t36271;
    (t36247, t36248, t36250, t36254, t36268, t36269, t36271, t36272)
}
