//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 740/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk740<F: Float>(t633: F, t7647: F, t707: F, t2143: F, t4710: F, t121: F, t168: F, t2149: F, t609: F, t4037: F, t623: F, t3153: F) -> (F, F, F, F, F) {
    let t7660 = t7647 * t633;
    let t7661 = t707 * t7660;
    let t7664 = t4710 * t2143;
    let t7665 = t121 * t7664;
    let t7668 = t168 * t2149;
    let t7669 = t7668 * t609;
    let t7670 = t707 * t7669;
    let t7671 = t4037 * t7670;
    let t7673 = t7668 * t623;
    let t7674 = t707 * t7673;
    let t7677 = t7668 * t633;
    let t7678 = t3153 * t7677;
    (t7661, t7665, t7671, t7674, t7678)
}
