//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 546/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk546(t3397: f64, t10: f64, t3161: f64, t39: f64, t65: f64) -> (f64, f64) {
    let t3398 = 4.166666666666667_f64 * t3397;
    let t3407 = t3161 * t10;
    let t3408 = t3407 * t39;
    let t3409 = t3408 * t65;
    (t3398, t3409)
}
