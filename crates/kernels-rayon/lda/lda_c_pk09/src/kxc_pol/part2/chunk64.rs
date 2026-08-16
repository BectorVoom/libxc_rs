//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 64/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk64(t66: f64, t77: f64, t88: f64, t142: f64) -> (f64, f64, f64, f64) {
    let t148 = 4.812726287291521_f64 * t66 + 0.64_f64 * t77 + 0.07519884823893001_f64;
    let t149 = f64::ln(t148);
    let t150 = t149 * t88;
    let t151 = t150 * t142;
    (t148, t149, t150, t151)
}
