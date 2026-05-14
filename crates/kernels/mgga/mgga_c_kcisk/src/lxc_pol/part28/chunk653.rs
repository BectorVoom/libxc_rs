//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 653/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk653<F: Float>(t2551: F, t4265: F, t4594: F, t702: F, t6759: F, t1919: F, t2063: F, t5254: F, t1797: F, t6764: F, t1920: F, t220: F, t140: F, t2554: F, t299: F, t2505: F, t695: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7368 = t4265 * t2551;
    let t7370 = t4594 * t702;
    let t7371 = t7370 * t6759;
    let t7375 = t1919 * t5254 * t2063;
    let t7378 = t1797 * t702;
    let t7379 = t7378 * t6764;
    let t7383 = t1919 * t1920 * t220;
    let t7387 = t140 * t299 * t2554;
    let t7389 = t2505 * t695;
    (t7368, t7370, t7371, t7375, t7378, t7379, t7383, t7387, t7389)
}
