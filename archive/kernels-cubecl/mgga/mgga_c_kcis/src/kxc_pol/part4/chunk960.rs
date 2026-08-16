//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 960/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk960<F: Float>(t2861: F, t3179: F, t251: F, t88: F, t304: F, t86: F, t1123: F, t1085: F, t3225: F, t3221: F, t1094: F, t3168: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t9524 = t2861 * t3179;
    let t9526 = t88 * t251;
    let t9528 = t86 * t9526 * t304;
    let t9529 = t9528 * t1123;
    let t9531 = t1085 * t3225;
    let t9532 = t9531 * sigma0;
    let t9536 = t2861 * t3221;
    let t9538 = t3168 * t1094;
    (t9524, t9526, t9528, t9529, t9531, t9532, t9536, t9538)
}
