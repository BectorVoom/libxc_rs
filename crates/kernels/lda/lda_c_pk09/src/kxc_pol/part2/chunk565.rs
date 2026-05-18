//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 565/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk565<F: Float>(t3176: F, t3767: F, t2971: F, t956: F, t3194: F, t2974: F, t1062: F, t789: F, t721: F, t3397: F, t3409: F, t3332: F) -> (F, F, F, F, F, F, F, F) {
    let t3768 = t3767 * t3176;
    let t3772 = t956 * t2971;
    let t3773 = t3772 * t3194;
    let t3775 = t3772 * t2974;
    let t3777 = t789 * t1062;
    let t3778 = t3777 * t721;
    let t3789 = F::new(0.15124939527727072) * t3397;
    let t3792 = F::new(0.6806222787477182) * t3409;
    let t3793 = F::new(0.06033977866125206) * t3332;
    (t3768, t3772, t3773, t3775, t3778, t3789, t3792, t3793)
}
