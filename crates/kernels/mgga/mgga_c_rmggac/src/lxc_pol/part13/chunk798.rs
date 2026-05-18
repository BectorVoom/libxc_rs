//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 798/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk798<F: Float>(t36268: F, t7198: F, t7197: F, t899: F, t271: F, t3899: F, t638: F, t641: F, t36293: F, t739: F, t36247: F, t35979: F, t4044: F) -> (F, F, F, F, F, F) {
    let t36976 = t7198 * t36268;
    let t36978 = t899 * t7197;
    let t36983 = t638 * t3899 * t271 * t641;
    let t36998 = t739 * t36293;
    let t37000 = t739 * t36247;
    let t37006 = t4044 * t35979;
    (t36976, t36978, t36983, t36998, t37000, t37006)
}
