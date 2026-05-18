//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 702/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk702<F: Float>(t4990: F, t533: F, t1782: F, t6579: F, t1836: F, t6590: F, t1853: F, t6292: F, t1856: F, t6477: F, t1877: F, t1808: F) -> (F, F, F, F, F, F) {
    let t6724 = t533 * t4990;
    let t6725 = t6724 * t1782;
    let t6727 = F::new(21.324527244551554) * t6725 * t6579;
    let t6729 = F::new(9.477567664245134) * t1836 * t6590;
    let t6735 = F::new(2.427516195194328) * t1853 * t6292;
    let t6736 = t1856 * t6477;
    let t6739 = F::new(19.489173774580152) * t1877 * t6292;
    let t6740 = t1808 * t6477;
    (t6727, t6729, t6735, t6736, t6739, t6740)
}
