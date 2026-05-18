//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 704/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk704<F: Float>(t485: F, t6780: F, t132: F, t4977: F, t93: F, t1992: F, t476: F, t6477: F, t2000: F, t6292: F, t1848: F, t747: F) -> (F, F, F, F, F, F, F, F) {
    let t6781 = t6780 * t485;
    let t6789 = t132 * t4977;
    let t6790 = t93 * t6789;
    let t6791 = t1992 * t6790;
    let t6792 = F::new(5.40024514194619) * t6791;
    let t6793 = t476 * t6477;
    let t6803 = t2000 * t6292;
    let t6804 = F::new(22.07984838129906) * t6803;
    let t6805 = t747 * t1848;
    (t6781, t6790, t6791, t6792, t6793, t6803, t6804, t6805)
}
