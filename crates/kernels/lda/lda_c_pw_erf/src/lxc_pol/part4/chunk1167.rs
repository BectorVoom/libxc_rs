//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1167/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1167<F: Float>(t1289: F, t6601: F, t9931: F, t9934: F, t9936: F, t9939: F, t9941: F, t9944: F, t9947: F, t9949: F, t11695: F, t11709: F, t11711: F, t11713: F, t11715: F, t11751: F, t11753: F, t11755: F, t11757: F, t11762: F, t11764: F, t11770: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17179 = 4.0 / 15.0 * t6601 * t1289;
    let t17180 = 8.0 / 135.0 * t9931;
    let t17181 = 32.0 / 405.0 * t9934;
    let t17182 = 8.0 / 135.0 * t9936;
    let t17183 = 4.0 / 135.0 * t9939;
    let t17184 = 16.0 / 135.0 * t9941;
    let t17185 = 8.0 / 135.0 * t9944;
    let t17186 = 32.0 / 405.0 * t9947;
    let t17187 = 8.0 / 135.0 * t9949;
    let t17200 = -0.009876543209876543 * t11695 + 0.12797037037037037 * t11709 + 0.015996296296296297 * t11711 + 0.026660493827160493 * t11713 + 0.06398518518518519 * t11715 - 0.047988888888888886 * t11751 - 0.04265679012345679 * t11753 - 0.09597777777777777 * t11755 + 0.011851851851851851 * t11757 - 0.017777777777777778 * t11762 - 0.07111111111111111 * t11764 - 0.19195555555555555 * t11770;
    (t17179, t17180, t17181, t17182, t17183, t17184, t17185, t17186, t17187, t17200)
}
