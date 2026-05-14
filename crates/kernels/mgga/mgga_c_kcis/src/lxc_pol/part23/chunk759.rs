//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 759/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk759<F: Float>(t14954: F, t85: F, t119: F, t41: F, t12274: F, t2003: F, t1396: F, t531: F, t1395: F, t5780: F, t6019: F, t1498: F, t1464: F, t11783: F, t2002: F, t3954: F, t5632: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14955 = t85 * t14954;
    let t15007 = t119 * t41;
    let t15008 = t85 * t15007;
    let t15800 = t12274 * t2003;
    let t15802 = t1396 * t531;
    let t15803 = t1395 * t15802;
    let t15804 = t5780 * t15803;
    let t15808 = t6019 * sigma2;
    let t15809 = t15808 * t1498;
    let t15810 = t1464 * t15809;
    let t15812 = t11783 * t2002;
    let t15813 = t1464 * t15812;
    let t15815 = t5632 * t3954;
    (t14955, t15008, t15800, t15802, t15804, t15808, t15810, t15813, t15815)
}
