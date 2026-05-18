//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 658/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk658<F: Float>(t5834: F, t5845: F, t319: F, t5759: F, t1634: F, t5569: F, t1336: F, t1580: F, t1625: F, t318: F, t5420: F, t1623: F, t5755: F) -> (F, F, F, F, F, F) {
    let t5847 = t5845 * t5834 / F::new(3.0);
    let t5854 = t319 * t5759;
    let t5856 = t1634 * t5569;
    let t5864 = t1580 * t1336;
    let t5865 = t5864 * t1625;
    let t5867 = t318 * t5420;
    let t5868 = t5867 * t1625;
    let t5871 = t1623 * t5755 / F::new(6.0);
    (t5847, t5854, t5856, t5865, t5868, t5871)
}
