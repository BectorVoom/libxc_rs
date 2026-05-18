//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 123/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk123<F: Float>(t372: F, t94: F, t333: F, t294: F, t305: F) -> (F, F, F, F, F) {
    let t373 = t94 * t372;
    let t374 = t333 * t373;
    let t378 = F::new(1.5625) * t294 + F::new(0.3208669506079574);
    let t381 = f64::atan(F::new(0.16004110557090126) / t378);
    let t382 = t381 * t305;
    (t373, t374, t378, t381, t382)
}
