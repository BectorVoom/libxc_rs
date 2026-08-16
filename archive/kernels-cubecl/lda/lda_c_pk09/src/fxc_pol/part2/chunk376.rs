//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 376/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk376<F: Float>(t1800: F, t1832: F, t1468: F, t533: F, t1782: F) -> (F, F, F) {
    let t1834 = F::cast_from(3.7610742193750633_f64) * t1832 * t1800;
    let t1835 = t533 * t1468;
    let t1836 = t1835 * t1782;
    (t1834, t1835, t1836)
}
