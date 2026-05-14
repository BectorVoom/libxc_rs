//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1226/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1226<F: Float>(t55867: F, t9425: F, t21499: F, t32025: F, t1390: F, t3924: F, t53214: F, t9446: F, t9453: F, t3532: F, t32018: F, t32095: F, t39810: F, t403: F, t1292: F, t1308: F, t13437: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110219 = t9425 * t55867;
    let t110222 = t32025 * t21499;
    let t110225 = t3924 * t1390;
    let t110256 = t9446 * t53214 * t9453;
    let t110379 = t3924 * t3532;
    let t110384 = t32018 * t21499;
    let t110423 = t32095 * t21499;
    let t110435 = t403 * t39810;
    let t110463 = t13437 * t1292 * t1308;
    (t110219, t110222, t110225, t110256, t110379, t110384, t110423, t110435, t110463)
}
