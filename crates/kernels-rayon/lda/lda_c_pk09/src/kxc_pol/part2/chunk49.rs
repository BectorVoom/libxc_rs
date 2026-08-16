//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 49/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk49(t79: f64, t66: f64, t77: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t101 = f64::ln(t79);
    let t104 = 1.1492271038405137_f64 * t66 + 0.15282509383508946_f64 * t77 + 0.01795667349750801_f64;
    let t105 = t101 * t104;
    let t106 = t105 * t89;
    (t101, t104, t105, t106)
}
