//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1226/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1226<F: Float>(t111028: F, t3373: F, t9368: F, t32633: F, t32637: F, t32669: F, t32672: F, t32664: F, t32647: F, t32646: F, t3368: F, t9382: F, t140: F, t15193: F, t190: F, t15629: F, t397: F, t9379: F, t9380: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t111065 = t3373 * t111028 * t9368;
    let t111067 = t32637 * t32633;
    let t111069 = t32669 * t32633;
    let t111071 = t32672 * t32633;
    let t111073 = t32664 * t32633;
    let t111075 = t32647 * t32633;
    let t111077 = t3368 * t32646;
    let t111078 = t111077 * t9382;
    let t111081 = t140 * t15193 * t190;
    let t111085 = t9379 * t397 * t9380 * t15629;
    (t111065, t111067, t111069, t111071, t111073, t111075, t111077, t111078, t111081, t111085)
}
