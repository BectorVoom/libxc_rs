//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1069/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1069<F: Float>(t10004: F, t11589: F, t10959: F, t11066: F, t11096: F, t11500: F, t11502: F, t11504: F, t11510: F, t11512: F, t11515: F, t11517: F, t11520: F, t11580: F, t11583: F, t11587: F, t2032: F, t2783: F, t455: F, t6594: F, t6981: F, t6982: F, t6984: F, t6985: F, t7293: F, t7304: F) -> F {
    let t11590 = t10004 * t11589;
    let t11594 = t11500 / F::new(18.0) - t11502 / F::new(6.0) - t11504 * t11096 / F::new(3.0) + t7304 * t2783 / F::new(6.0) + t11510 / F::new(6.0) + t11512 * t2032 / F::new(6.0) - t11515 / F::new(6.0) - t11517 * t11096 / F::new(3.0) + t11520 / F::new(18.0) + t7293 * t2783 / F::new(6.0) - F::new(0.10237773105191754) * t11066 - F::new(0.20475546210383508) * t10959 - t11580 * t455 / F::new(6.0) - t11583 * t455 / F::new(6.0) + t6981 + t11587 * t11590 / F::new(3.0) + t6982 + t6984 + t6985 - F::new(0.02466859483068398) * t6594;
    t11594
}
