//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 956/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk956(t5047: f64, t5071: f64, t5952: f64, t5965: f64, t5966: f64, t5971: f64, t5974: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9922: f64, t9925: f64, t9929: f64, t9933: f64, t9936: f64, t9943: f64) -> f64 {
    let t10082 = -t5966 + t5971 + t5952 + t5965 + 0.15282509383508946_f64 * t5047 - t5974 + 0.05094169794502982_f64 * t5071 + 1.532302805120685_f64 * t9922 - 1.532302805120685_f64 * t9925 - 1.532302805120685_f64 * t9929 + 2.2984542076810275_f64 * t9933 - 1.532302805120685_f64 * t9936 + 0.15282509383508946_f64 * t9746 + 0.05094169794502982_f64 * t9753 + 0.15282509383508946_f64 * t9756 + 0.30565018767017893_f64 * t9628 - 0.510767601706895_f64 * t9943;
    t10082
}
