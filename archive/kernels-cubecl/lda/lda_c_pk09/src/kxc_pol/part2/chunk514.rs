//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 514/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk514<F: Float>(t200: F, t2983: F, t242: F, t48: F, t56: F, t623: F, t92: F, t44: F, t618: F, t54: F, t633: F, t51: F, t628: F) -> (F, F, F, F, F, F, F) {
    let t2984 = t200 * t2983;
    let t2988 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t56 * t242 * t48;
    let t2990 = t56 * t92 * t623;
    let t2993 = F::cast_from(1.0_f64) / t618 / t44;
    let t3007 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t56 * t242 * t54;
    let t3009 = t56 * t92 * t633;
    let t3012 = F::cast_from(1.0_f64) / t628 / t51;
    (t2984, t2988, t2990, t2993, t3007, t3009, t3012)
}
