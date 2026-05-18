//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 456/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk456<F: Float>(t51: F, t2471: F, t278: F, t2457: F, t2465: F, zeta_threshold: F) -> F {
    let t52 = t51 <= zeta_threshold;
    let t2472 = t278 * t2471;
    let t2473 = piecewise3::<f64>(t52, t2457, t2472);
    let t2474 = t2465 + t2473;
    t2474
}
