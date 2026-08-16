//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 157/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk157(t429: f64, t489: f64, t435: f64, t441: f64, t305: f64) -> (f64, f64, f64, f64) {
    let t490 = t489 * t429;
    let t495 = 1.6042420957638404_f64 * t435 + 0.64_f64 * t441 + 0.07519884823893001_f64;
    let t496 = f64::ln(t495);
    let t497 = t496 * t305;
    (t490, t495, t496, t497)
}
