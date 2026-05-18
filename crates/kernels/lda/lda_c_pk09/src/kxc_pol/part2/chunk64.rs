//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 64/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk64<F: Float>(t66: F, t77: F, t88: F, t142: F) -> (F, F, F, F) {
    let t148 = F::new(4.812726287291521) * t66 + F::new(0.64) * t77 + F::new(0.07519884823893001);
    let t149 = f64::ln(t148);
    let t150 = t149 * t88;
    let t151 = t150 * t142;
    (t148, t149, t150, t151)
}
