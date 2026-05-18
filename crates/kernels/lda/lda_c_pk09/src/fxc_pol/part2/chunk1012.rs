//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1012/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1012<F: Float>(t2711: F, t4785: F, t1151: F, t2704: F, t1161: F, t1156: F, t4842: F, t9648: F, t9649: F, t9650: F, t9651: F, t420: F) -> (F, F, F, F) {
    let t10968 = t4785 * t2711;
    let t10974 = t1151 * t2704;
    let t10976 = t2704 * t1161;
    let t10977 = t1156 * t10976;
    let t10979 = t9648 + t9649 - t9650 - t9651 - t4842;
    let t10980 = t10979 * t420;
    (t10968, t10974, t10977, t10980)
}
