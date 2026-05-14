//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 550/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk550<F: Float>(t140: F, t430: F, t728: F, t1922: F, t4265: F, t1925: F, t299: F, t41: F, t4594: F, t4597: F, t702: F, t1860: F, t695: F, t1849: F, t1871: F, t1929: F) -> (F, F, F, F, F, F, F, F) {
    let t5242 = 0.88437037037037037037e-2 * t140 * t430 * t728;
    let t5243 = t4265 * t1922;
    let t5246 = t140 * t299 * t1925;
    let t5248 = t41 * t4594;
    let t5249 = t702 * t4597;
    let t5254 = t1860 * t695;
    let t5259 = t702 * t1849;
    let t5277 = t1929 * t1871;
    (t5242, t5243, t5246, t5248, t5249, t5254, t5259, t5277)
}
