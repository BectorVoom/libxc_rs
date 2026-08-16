//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 629/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk629(t5193: f64, t282: f64, t4993: f64, t68: f64, t286: f64) -> (f64, f64) {
    let t5194 = 2.6666666666666665_f64 * t5193;
    let t5207 = t4993 * t282 * t68;
    let t5208 = t5207 * t286;
    (t5194, t5208)
}
