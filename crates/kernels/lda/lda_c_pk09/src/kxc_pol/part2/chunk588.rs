//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 588/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk588<F: Float>(t4281: F, t90: F, t1106: F, t4361: F, t4093: F, t623: F, t896: F, t1063: F, t4365: F, t10: F, t104: F, t125: F) -> (F, F, F, F, F, F) {
    let t4480 = F::new(5.0) / F::new(27.0) * t90 * t4281;
    let t4489 = t1106 * t4361;
    let t4494 = t896 * t4093 * t623;
    let t4497 = t1063 * t4361;
    let t4499 = t1063 * t4365;
    let t4502 = t104 * t125 * t10;
    (t4480, t4489, t4494, t4497, t4499, t4502)
}
