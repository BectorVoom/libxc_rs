//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 673/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk673<F: Float>(t1422: F, t3549: F, t7706: F, t1423: F, t7710: F, t3558: F, t7757: F, t457: F, t2191: F, t5932: F, t3564: F, t1428: F, t7764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7858 = t1422 * t3549 * t7706;
    let t7862 = t1422 * t1423 * t7710;
    let t7865 = t3558 * t7757;
    let t7866 = t457 * t7865;
    let t7869 = t5932 * t2191;
    let t7870 = t3564 * t7869;
    let t7873 = t1428 * t7764;
    let t7874 = t457 * t7873;
    let t7877 = t2191 * t2191;
    (t7858, t7862, t7865, t7866, t7869, t7870, t7873, t7874, t7877)
}
