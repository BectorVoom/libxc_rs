//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 480/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk480<F: Float>(t200: F, t2983: F, t242: F, t48: F, t56: F, t623: F, t92: F, t44: F, t618: F, t54: F, t633: F, t51: F, t628: F, t143: F, t569: F, t933: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2984 = t200 * t2983;
    let t2988 = 2.0 / 9.0 * t56 * t242 * t48;
    let t2990 = t56 * t92 * t623;
    let t2993 = 1.0 / t618 / t44;
    let t3007 = 2.0 / 9.0 * t56 * t242 * t54;
    let t3009 = t56 * t92 * t633;
    let t3012 = 1.0 / t628 / t51;
    let t3032 = t143 * t2983;
    let t3034 = t933 * t569;
    (t2984, t2988, t2990, t2993, t3007, t3009, t3012, t3032, t3034)
}
