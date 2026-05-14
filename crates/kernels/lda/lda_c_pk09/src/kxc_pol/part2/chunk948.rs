//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 948/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk948<F: Float>(t2149: F, t7260: F, t93: F, t1805: F, t2759: F, t11679: F, t68: F, t1800: F, t2813: F, t6253: F, t2854: F, t305: F, t1819: F, t1947: F, t2855: F, t2042: F) -> (F, F, F, F, F, F) {
    let t11766 = t7260 * t2149;
    let t11767 = t93 * t11766;
    let t11773 = t2759 * t1805;
    let t11775 = t11679 * t68;
    let t11776 = t11775 * t1800;
    let t11778 = t2813 * t6253;
    let t11782 = t2854 * t305;
    let t11783 = t1819 * t11782;
    let t11786 = t2855 * t1947;
    let t11787 = t11786 * t2042;
    (t11767, t11773, t11776, t11778, t11783, t11787)
}
