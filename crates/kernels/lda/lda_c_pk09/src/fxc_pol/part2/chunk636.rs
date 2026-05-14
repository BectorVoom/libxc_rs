//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 636/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk636<F: Float>(t6579: F, t6725: F, t1836: F, t6590: F, t1853: F, t6292: F, t1856: F, t6477: F, t1877: F, t1808: F, t1672: F, t1820: F, t6319: F, t6325: F, t6464: F, t538: F, t6601: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6727 = 21.324527244551554 * t6725 * t6579;
    let t6729 = 9.477567664245134 * t1836 * t6590;
    let t6735 = 2.427516195194328 * t1853 * t6292;
    let t6736 = t1856 * t6477;
    let t6739 = 19.489173774580152 * t1877 * t6292;
    let t6740 = t1808 * t6477;
    let t6743 = t1820 * t1672;
    let t6747 = 11.879313099038017 * t6319;
    let t6749 = 7.919542066025344 * t6325;
    let t6755 = 2.6398473553417814 * t6464;
    let t6764 = 0.9840332968370255 * t538 * t6601;
    (t6727, t6729, t6735, t6736, t6739, t6740, t6743, t6747, t6749, t6755, t6764)
}
