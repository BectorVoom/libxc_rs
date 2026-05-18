//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 496/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk496<F: Float>(t2026: F, t2027: F, t2029: F, t2030: F, t2044: F, t2047: F, t2058: F, t2060: F, t2108: F, t2110: F, t2745: F, t2749: F, t2753: F, t2783: F, t453: F, t472: F) -> F {
    let t2791 = -t2026 - t2027 - t2029 - t2030 - t472 * t2783 / F::new(6.0) + t453 * t2783 / F::new(6.0) + t2044 - t2047 + t2058 + F::new(0.037002892246025966) * t2745 - F::new(0.037002892246025966) * t2749 - F::new(0.14975624337724558) * t2753 + t2060 - t2108 + t2110;
    t2791
}
