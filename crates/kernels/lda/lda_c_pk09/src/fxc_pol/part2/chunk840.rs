//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 840/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk840<F: Float>(t10020: F, t5683: F, t318: F, t332: F, t2520: F, t623: F, t333: F) -> (F, F, F) {
    let t10021 = t5683 * t10020;
    let t10023 = t318 * t332;
    let t10024 = t2520 * t623;
    let t10025 = t333 * t10024;
    (t10021, t10023, t10025)
}
