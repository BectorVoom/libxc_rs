//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 448/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk448(t1000: f64, t1001: f64, t2159: f64, t2163: f64, t2167: f64, t2171: f64, t2175: f64, t2179: f64, t995: f64, t996: f64, t101: f64, t89: f64) -> (f64, f64, f64) {
    let t2392 = t995 + t996 + 2.2984542076810275_f64 * t2159 + 2.2984542076810275_f64 * t2163 - 2.2984542076810275_f64 * t2167 + t1000 + t1001 + 0.15282509383508946_f64 * t2171 + 0.15282509383508946_f64 * t2175 - 0.15282509383508946_f64 * t2179;
    let t2393 = t101 * t2392;
    let t2394 = t2393 * t89;
    (t2392, t2393, t2394)
}
