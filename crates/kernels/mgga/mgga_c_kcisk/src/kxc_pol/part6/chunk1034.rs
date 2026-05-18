//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1034/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1034<F: Float>(t1341: F, t30290: F, t1340: F, t3759: F, t2231: F, t7710: F, t3797: F, t3796: F, t3482: F, t2152: F, t3485: F, t3484: F) -> (F, F, F) {
    let t30967 = t1341 * t30290;
    let t30968 = t1340 * t30967;
    let t30969 = t3759 * t30968;
    let t30972 = t7710 * t2231;
    let t30973 = t3797 * t30972;
    let t30974 = t3796 * t30973;
    let t30975 = t3482 * t30974;
    let t30978 = t3485 * t7710 * t2152;
    let t30979 = t3484 * t30978;
    (t30969, t30975, t30979)
}
