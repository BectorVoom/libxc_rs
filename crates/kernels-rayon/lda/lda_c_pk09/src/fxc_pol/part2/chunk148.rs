//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 148/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk148(t443: f64, t435: f64, t441: f64) -> (f64, f64, f64) {
    let t458 = f64::ln(t443);
    let t461 = 0.38307570128017127_f64 * t435 + 0.15282509383508946_f64 * t441 + 0.01795667349750801_f64;
    let t462 = t458 * t461;
    (t458, t461, t462)
}
