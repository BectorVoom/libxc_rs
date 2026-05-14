//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 664/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk664<F: Float>(t633: F, t7647: F, t707: F, t2143: F, t4710: F, t121: F, t168: F, t2149: F, t609: F, t4037: F, t623: F, t3153: F, t2984: F, t3032: F, t3034: F, t3090: F, t3102: F, t3105: F, t3107: F, t3120: F, t4715: F, t4725: F) -> (F,) {
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
    let t7686 = -0.04115066352984959 * t4715 * t7661 + 0.04115066352984959 * t4715 * t7665 - 0.04115066352984959 * t7671 - 0.04115066352984959 * t4715 * t7674 - 0.08230132705969918 * t4725 * t7678 + 1.6183441301295518 * t2984 + 2.507382812916709 * t3032 + 0.4178971354861182 * t3034 + t3090 + t3102 - 2.400108951976084 * t3105 - 2.400108951976084 * t3107 - t3120;
    (t7686,)
}
