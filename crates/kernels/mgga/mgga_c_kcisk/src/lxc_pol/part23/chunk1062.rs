//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1062/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1062<F: Float>(t2259: F, t4309: F, t2257: F, t3783: F, t4211: F, t1482: F, t469: F, t6318: F, t6387: F, t4205: F, t19715: F, t470: F, t487: F, t1487: F, t19796: F, t4204: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t21312 = t2259 * t4309;
    let t21314 = t2257 * t3783;
    let t21315 = t21314 * sigma0;
    let t21316 = t21315 * t4211;
    let t21318 = t1482 * t469;
    let t21319 = t21318 * t6318;
    let t21321 = t6387 * t469;
    let t21322 = t21321 * t4205;
    let t21324 = t470 * t19715;
    let t21325 = t487 * t21324;
    let t21326 = t1487 * t21325;
    let t21328 = t4204 * t19796;
    (t21312, t21314, t21316, t21319, t21322, t21326, t21328)
}
