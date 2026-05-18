//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 659/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk659<F: Float>(t304: F, t5009: F, t5834: F, t1642: F, t5838: F, t1623: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F, t5212: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5878 = t304 * t5009;
    let t5880 = t5878 * t5834 / F::new(3.0);
    let t5884 = t1642 * t5838 / F::new(9.0);
    let t5886 = t1623 * t5838 / F::new(9.0);
    let t5903 = F::new(0.3056501876701794) * t5039;
    let t5907 = F::new(2.0430704068275776) * t5161;
    let t5916 = F::new(0.2037667917801196) * t5045;
    let t5917 = F::new(0.17025586723563146) * t5190;
    let t5922 = F::new(1.5323028051206833) * t5208;
    let t5923 = F::new(1.5323028051206833) * t5212;
    (t5880, t5884, t5886, t5903, t5907, t5916, t5917, t5922, t5923)
}
