//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 909/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk909<F: Float>(t3508: F, t7907: F, t1411: F, t2152: F, t5967: F, t1450: F, t1415: F, t14255: F, t25337: F, t3484: F, t5633: F, t3739: F, t8074: F, t14199: F, t8171: F, t3482: F) -> (F, F, F, F, F, F, F) {
    let t25362 = t3508 * t7907;
    let t25363 = t1411 * t25362;
    let t25365 = t5967 * t2152;
    let t25366 = t1450 * t25365;
    let t25367 = t1415 * t25366;
    let t25368 = t1411 * t25367;
    let t25370 = t14255 * t25337;
    let t25371 = t3484 * t25370;
    let t25372 = t5633 * t25371;
    let t25376 = t3739 * t8074;
    let t25380 = t14199 * t8171;
    let t25381 = t3482 * t25380;
    (t25363, t25365, t25368, t25370, t25372, t25376, t25381)
}
