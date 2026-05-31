//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 531/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk531<F: Float>(t2971: F, t809: F, t3194: F, t2974: F, t1062: F, t975: F, t721: F, t943: F, t150: F, t119: F, t805: F, t1123: F, t79: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3254 = t809 * t2971;
    let t3255 = t3254 * t3194;
    let t3257 = t3254 * t2974;
    let t3259 = t975 * t1062;
    let t3260 = t3259 * t721;
    let t3262 = t943 * t2971;
    let t3263 = t3262 * t3194;
    let t3265 = t150 * t2971;
    let t3268 = t805 * t119;
    let t3272 = F::cast_from(1.0_f64) / t1123 / t79;
    (t3254, t3255, t3257, t3260, t3262, t3263, t3265, t3268, t3272)
}
