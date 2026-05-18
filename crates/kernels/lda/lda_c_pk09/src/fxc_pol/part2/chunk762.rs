//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 762/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk762<F: Float>(t2143: F, t56: F, t92: F, t2149: F, t14: F, t2988: F, t2990: F, t3007: F, t3009: F, t7704: F, t7766: F) -> F {
    let t7821 = t56 * t92 * t2143;
    let t7827 = t56 * t92 * t2149;
    let t7831 = -t2988 + t2990 / F::new(3.0) + t7821 / F::new(3.0) + t56 * t14 * t7766 - t3007 + t3009 / F::new(3.0) + t7827 / F::new(3.0) + t56 * t14 * t7704;
    t7831
}
