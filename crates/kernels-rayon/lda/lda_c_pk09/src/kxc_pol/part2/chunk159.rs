//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 159/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk159(t429: f64, t501: f64, t466: f64, t305: f64) -> (f64, f64, f64, f64) {
    let t502 = t501 * t429;
    let t505 = t466 + 0.4822571819944727_f64;
    let t506 = f64::ln(t505);
    let t507 = t506 * t305;
    (t502, t505, t506, t507)
}
