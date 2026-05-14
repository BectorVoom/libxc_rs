//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 390/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk390<F: Float>(t1873: F, t2528: F, t1869: F, t2454: F, t719: F, t717: F, t415: F, t1899: F, t2441: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t2529 = t1873 * t2528;
    let t2530 = t1869 * t2529;
    let t2532 = sigma2 * t2454;
    let t2533 = t2532 * t719;
    let t2534 = t717 * t2533;
    let t2535 = t415 * t2534;
    let t2537 = t1899 * t2441;
    (t2529, t2530, t2532, t2533, t2534, t2535, t2537)
}
