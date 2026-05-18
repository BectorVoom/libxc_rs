//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 172/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk172<F: Float>(t133: F, t568: F, t128: F, t242: F, t94: F) -> (F, F, F, F) {
    let t569 = t568 * t133;
    let t570 = t128 * t569;
    let t571 = F::new(1.200054475988042) * t570;
    let t572 = t242 * t94;
    (t569, t570, t571, t572)
}
