//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 867/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk867<F: Float>(t13949: F, t4231: F, t4230: F, t1492: F, t4210: F, t4209: F, t1512: F, t4192: F, t493: F, t1481: F, t3783: F, t4211: F, t1501: F, t4185: F, t1483: F, t4241: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t14312 = t4231 * t13949;
    let t14313 = t4230 * t14312;
    let t14315 = t1492 * t4210;
    let t14316 = t4209 * t14315;
    let t14317 = t1512 * t4192;
    let t14318 = t493 * t14317;
    let t14320 = t1481 * t3783;
    let t14321 = t14320 * sigma0;
    let t14322 = t14321 * t4211;
    let t14324 = t1501 * t4185;
    let t14326 = t1483 * t4241;
    (t14313, t14316, t14318, t14322, t14324, t14326)
}
