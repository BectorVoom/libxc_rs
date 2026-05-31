//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 703/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk703<F: Float>(t1672: F, t1820: F, t6319: F, t6325: F, t6464: F, t538: F, t6601: F, t1146: F, t132: F, t142: F, t550: F, t2005: F, t443: F) -> (F, F, F, F, F, F, F) {
    let t6743 = t1820 * t1672;
    let t6747 = F::cast_from(11.879313099038017_f64) * t6319;
    let t6749 = F::cast_from(7.919542066025344_f64) * t6325;
    let t6755 = F::cast_from(2.6398473553417814_f64) * t6464;
    let t6764 = F::cast_from(0.9840332968370255_f64) * t538 * t6601;
    let t6769 = t142 * t1146 * t132;
    let t6771 = F::cast_from(3.948986526768806_f64) * t550 * t6769;
    let t6780 = F::cast_from(1.0_f64) / t2005 / t443;
    (t6743, t6747, t6749, t6755, t6764, t6771, t6780)
}
