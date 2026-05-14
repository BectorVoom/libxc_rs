//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 916/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk916<F: Float>(t6931: F, t960: F, t16026: F, t1835: F, t16004: F, t706: F, t16013: F, t15999: F, t1919: F, t2497: F, t3119: F, t2502: F, t3123: F, t2494: F, t3114: F, t156: F, t5822: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16190 = t960 * t6931;
    let t16192 = t1835 * t16026;
    let t16195 = t706 * t16004;
    let t16198 = t706 * t16013;
    let t16201 = t1919 * t15999;
    let t16204 = t3119 * t2497;
    let t16206 = t3123 * t2502;
    let t16208 = t3114 * t2494;
    let t16210 = t156 * t5822;
    (t16190, t16192, t16195, t16198, t16201, t16204, t16206, t16208, t16210)
}
