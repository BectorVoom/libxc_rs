//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1261/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1261<F: Float>(t4169: F, t9481: F, t14398: F, t394: F, t1486: F, t3913: F, t1299: F, t4214: F, t14364: F, t1482: F, t32277: F, t3507: F, t4208: F, t1511: F, t1414: F, t14320: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t109171 = t9481 * t4169;
    let t109217 = t14398 * t394;
    let t109226 = t1486 * t3913;
    let t109229 = t4214 * t1299;
    let t109241 = t14364 * t394;
    let t109270 = t1482 * t1299;
    let t109279 = t3507 * t32277;
    let t109287 = t4208 * t1299;
    let t109293 = sigma0 * t1511;
    let t109294 = t1414 * t109293;
    let t109297 = t14320 * t394;
    (t109171, t109217, t109226, t109229, t109241, t109270, t109279, t109287, t109293, t109294, t109297)
}
