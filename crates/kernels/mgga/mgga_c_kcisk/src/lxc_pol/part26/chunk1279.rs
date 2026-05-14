//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1279/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1279<F: Float>(t1308: F, t388: F, t54621: F, t1220: F, t6147: F, t19972: F, t3930: F, t33459: F, t3969: F, t1292: F, t6221: F, t109882: F, t470: F, t32042: F, t33373: F, t32022: F, t33451: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114195 = t54621 * t388 * t1308;
    let t114199 = t1220 * t6147 * t1308;
    let t114205 = t19972 * t388 * t1308;
    let t114209 = t3930 * t6147 * t1308;
    let t114225 = t33459 * t3969;
    let t114231 = t6221 * t1292 * t1308;
    let t114243 = t109882 * t470;
    let t114264 = t33373 * t32042;
    let t114271 = 0.18518518518518518519e-1 * t32022 * t33451;
    (t114195, t114199, t114205, t114209, t114225, t114231, t114243, t114264, t114271)
}
