//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 863/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk863<F: Float>(t7900: F, t7917: F, t7896: F, t7904: F, t7908: F, t7913: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> F {
    let t8929 = F::new(24.0) * t7900;
    let t8933 = F::new(24.0) * t7917;
    let t8942 = F::new(1.642838787112742) * t7896 - t8929 - F::new(24.0) * t7904 - F::new(24.0) * t7908 + F::new(36.0) * t7913 + t8933 + F::new(0.821419393556371) * t7919 + F::new(0.821419393556371) * t7923 + F::new(0.821419393556371) * t7926 + F::new(0.821419393556371) * t7928 + F::new(0.821419393556371) * t7931 + F::new(0.821419393556371) * t7935 + F::new(0.5476129290375806) * t7939 + F::new(0.5476129290375806) * t7942;
    t8942
}
