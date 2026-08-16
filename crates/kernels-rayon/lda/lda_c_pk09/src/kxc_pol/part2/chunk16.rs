//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 16/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk16(t34: f64, t10: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t35 = t34 * t34;
    let t36 = t35 * t10;
    let t37 = pi * pi;
    let t38 = pow_1_3(t37);
    (t35, t36, t37, t38)
}
