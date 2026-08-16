//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 15/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk15(t21: f64, t17: f64, t19: f64, t18: f64, t9: f64) -> (f64, f64, f64, f64) {
    let t22 = pow_1_4(4.0_f64);
    let t23 = t22 * t22;
    let t24 = t23 * t22;
    let t25 = t21 * t24;
    let t26 = pow_1_4(t17);
    let t30 = f64::exp(-0.25916439866088_f64 * t19);
    let t34 = 0.538074483500437_f64 - 0.5565237477462975_f64 * t25 * t26 + 0.6549274647407946_f64 * t30 * t9 * t18;
    (t24, t26, t30, t34)
}
