//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1220/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1220<F: Float>(t14364: F, t394: F, t32277: F, t3507: F, t1299: F, t4208: F, t1511: F, t1414: F, t14320: F, t14293: F, t2726: F, t4350: F, t4374: F, t14612: F, t1588: F, t1390: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t109241 = t14364 * t394;
    let t109279 = t3507 * t32277;
    let t109287 = t4208 * t1299;
    let t109293 = sigma0 * t1511;
    let t109294 = t1414 * t109293;
    let t109297 = t14320 * t394;
    let t109321 = t2726 * t14293;
    let t109378 = t4374 * t4350;
    let t109390 = t1588 * t14612;
    let t109420 = t3507 * t394;
    let t109475 = t4350 * t1390;
    (t109241, t109279, t109287, t109294, t109297, t109321, t109378, t109390, t109420, t109475)
}
