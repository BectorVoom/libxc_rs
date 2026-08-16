//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 533/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk533(t3290: f64, t957: f64, t3223: f64, t944: f64, t611: f64, t625: f64) -> (f64, f64, f64, f64) {
    let t3292 = 2.427516195194328_f64 * t957 * t3290;
    let t3300 = t944 * t3223;
    let t3303 = 1.8805371096875316_f64 * t944 * t3290;
    let t3317 = t611 * t625;
    (t3292, t3300, t3303, t3317)
}
