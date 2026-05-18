//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 556/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk556<F: Float>(t158: F, t10: F, t733: F, t93: F, t169: F, t3161: F, t96: F, t3118: F, t841: F, t155: F, t3230: F, t3233: F) -> (F, F, F, F, F, F, F) {
    let t3516 = t158 * t158;
    let t3517 = F::new(1.0) / t3516;
    let t3522 = t733 * t10;
    let t3523 = t3522 * t93;
    let t3525 = t96 * t169 * t3161;
    let t3527 = F::new(0.08230132705969918) * t3523 * t3525;
    let t3529 = F::new(0.05486755137313279) * t3118 * t841;
    let t3534 = t155 * t3230;
    let t3536 = t155 * t3233;
    (t3517, t3522, t3523, t3527, t3529, t3534, t3536)
}
