//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 554/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk554(t142: f64, t3494: f64, t3163: f64, t572: f64, t720: f64) -> (f64, f64) {
    let t3495 = t3494 * t142;
    let t3497 = 37.27051603526593_f64 * t3495 * t3163;
    let t3498 = t572 * t720;
    (t3497, t3498)
}
